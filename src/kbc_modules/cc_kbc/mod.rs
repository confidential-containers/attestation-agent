// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use crate::{
    common::crypto::decrypt,
    kbc_modules::{KbcCheckInfo, KbcInterface, ResourceDescription},
};

pub mod attester;
mod crypto;
mod kbs_protocol;

use anyhow::*;
use async_trait::async_trait;
use attester::{detect_tee_type, Attester};
use core::time::Duration;
use crypto::{hash_chunks, TeeKey};
use kbs_protocol::message::*;
use kbs_types::ErrorInformation;
use zeroize::Zeroizing;

use super::AnnotationPacket;

const KBS_REQ_TIMEOUT_SEC: u64 = 60;
const KBS_GET_RESOURCE_MAX_ATTEMPT: u64 = 3;

pub const KBS_URL_PREFIX: &str = "kbs/v0";

pub struct Kbc {
    tee: String,
    kbs_uri: String,
    token: Option<String>,
    nonce: String,
    tee_key: Option<TeeKey>,
    attester: Option<Box<dyn Attester + Send + Sync>>,
    http_client: reqwest::Client,
    authenticated: bool,
}

pub fn key_url(kid: &str) -> Result<String> {
    let kid_without_prefix = kid.split("://").collect::<Vec<&str>>()[1].to_string();
    let (kbs_addr, key_path) = kid_without_prefix
        .split_once('/')
        .ok_or_else(|| anyhow!("Invalid KID in AnnotationPacket"))?;

    // Now only support `http://` prefix.
    Ok(format!(
        "http://{kbs_addr}/{KBS_URL_PREFIX}/resource/{key_path}"
    ))
}

#[async_trait]
impl KbcInterface for Kbc {
    fn check(&self) -> Result<KbcCheckInfo> {
        Err(anyhow!("Check API of this KBC is unimplemented."))
    }

    async fn decrypt_payload(&mut self, annotation_packet: AnnotationPacket) -> Result<Vec<u8>> {
        let key_url = key_url(&annotation_packet.kid)?;
        if !key_url.starts_with(&self.kbs_uri) {
            bail!(
                "Multi-KBS resource is not supported, Unmatch KBS address: {}",
                &key_url
            )
        }

        let response = self.request_kbs_resource(key_url).await?;
        let key = Zeroizing::new(self.decrypt_response_output(response)?);

        decrypt(
            key,
            base64::decode(annotation_packet.wrapped_data)?,
            base64::decode(annotation_packet.iv)?,
            &annotation_packet.wrap_type,
        )
    }

    #[allow(unused_assignments)]
    async fn get_resource(&mut self, description: &str) -> Result<Vec<u8>> {
        let desc: ResourceDescription = serde_json::from_str::<ResourceDescription>(description)?;
        let mut resource_url = String::default();
        let resource_type = desc
            .optional
            .get("type")
            .ok_or_else(|| anyhow!("Invalid Resource description: Missing `type` field"))?;
        let resource_tag = desc
            .optional
            .get("tag")
            .ok_or_else(|| anyhow!("Invalid Resource description: Missing `tag` field"))?;
        if let Some(resource_repo) = desc.optional.get("repository") {
            resource_url = format!(
                "{}/resource/{resource_repo}/{resource_type}/{resource_tag}",
                self.kbs_uri
            );
        } else {
            resource_url = format!("{}/resource/{resource_type}/{resource_tag}", self.kbs_uri);
        }

        let response = self.request_kbs_resource(resource_url).await?;

        self.decrypt_response_output(response)
    }
}

impl Kbc {
    pub fn new(kbs_uri: String) -> Kbc {
        // Detect TEE type of the current platform.
        let tee_type = detect_tee_type();

        // Create attester instance.
        let attester = tee_type.to_attester().ok();

        Kbc {
            tee: tee_type.to_string(),
            kbs_uri: format!("{kbs_uri}/{KBS_URL_PREFIX}"),
            token: None,
            nonce: String::default(),
            tee_key: TeeKey::new().ok(),
            attester,
            http_client: build_http_client().unwrap(),
            authenticated: false,
        }
    }

    fn generate_evidence(&self) -> Result<Attestation> {
        let key = self
            .tee_key
            .as_ref()
            .ok_or_else(|| anyhow!("Generate TEE key failed"))?;
        let attester = self
            .attester
            .as_ref()
            .ok_or_else(|| anyhow!("TEE attester missed"))?;

        let tee_pubkey = key
            .export_pubkey()
            .map_err(|e| anyhow!("Export TEE pubkey failed: {:?}", e))?;

        let ehd_chunks = vec![
            self.nonce.clone().into_bytes(),
            tee_pubkey.k.clone().into_bytes(),
        ];

        let ehd = hash_chunks(ehd_chunks);

        let tee_evidence = attester
            .get_evidence(ehd)
            .map_err(|e| anyhow!("Get TEE evidence failed: {:?}", e))?;

        Ok(Attestation {
            tee_pubkey,
            tee_evidence,
        })
    }

    fn decrypt_response_output(&self, response: Response) -> Result<Vec<u8>> {
        let key = self
            .tee_key
            .clone()
            .ok_or_else(|| anyhow!("TEE rsa key missing"))?;
        response.decrypt_output(key)
    }

    fn tee(&self) -> &str {
        &self.tee
    }

    fn kbs_uri(&self) -> &str {
        &self.kbs_uri
    }

    fn http_client(&mut self) -> &mut reqwest::Client {
        &mut self.http_client
    }

    async fn establish_kbs_session(&mut self) -> Result<()> {
        let kbs_uri = self.kbs_uri().to_string();

        let challenge = self
            .http_client()
            .post(format!("{kbs_uri}/auth"))
            .header("Content-Type", "application/json")
            .json(&Request::new(self.tee().to_string()))
            .send()
            .await?
            .json::<Challenge>()
            .await?;
        self.nonce = challenge.nonce.clone();

        let attest_response = self
            .http_client()
            .post(format!("{kbs_uri}/attest"))
            .header("Content-Type", "application/json")
            .json(&self.generate_evidence()?)
            .send()
            .await?;

        match attest_response.status() {
            reqwest::StatusCode::OK => {
                self.authenticated = true;
                Ok(())
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                let error_info = attest_response.json::<ErrorInformation>().await?;
                bail!("KBS attest unauthorized, Error Info: {:?}", error_info)
            }
            _ => {
                bail!(
                    "KBS Server Internal Failed, Response: {:?}",
                    attest_response.text().await?
                )
            }
        }
    }

    async fn request_kbs_resource(&mut self, resource_url: String) -> Result<Response> {
        for attempt in 1..=KBS_GET_RESOURCE_MAX_ATTEMPT {
            log::info!("CC-KBC: trying to get resource, attempt {attempt}");

            if !self.authenticated {
                self.establish_kbs_session().await?;
            }

            let res = self.http_client().get(&resource_url).send().await?;

            match res.status() {
                reqwest::StatusCode::OK => {
                    let response = res.json::<Response>().await?;
                    return Ok(response);
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    self.authenticated = false;
                    continue;
                }
                reqwest::StatusCode::NOT_FOUND => {
                    bail!("KBS resource Not Found (Error 404)")
                }
                _ => {
                    bail!(
                        "KBS Server Internal Failed, Response: {:?}",
                        res.text().await?
                    )
                }
            }
        }

        bail!("Request KBS resource: Attested but KBS still return Unauthorized")
    }
}

fn build_http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .cookie_store(true)
        .user_agent(format!(
            "attestation-agent-cc-kbc/{}",
            env!("CARGO_PKG_VERSION")
        ))
        .timeout(Duration::from_secs(KBS_REQ_TIMEOUT_SEC))
        .build()
        .map_err(|e| anyhow!("Build KBS http client failed: {:?}", e))
}
