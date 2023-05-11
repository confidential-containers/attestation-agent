// Copyright (c) 2023 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

//! This is a sample KMS implementation example.
//!
//! This sample KMS uses a hardcoded [`ROOT_SECRET`] to work as the entropy source
//! of a fake "HSM". All keys, IV are derived from a uuid and the [`ROOT_SECRET`].

use std::collections::HashSet;

use anyhow::*;
use async_trait::async_trait;
use crypto::WrapType;
use hkdf::Hkdf;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeroize::Zeroizing;

use crate::KMS;

/// This secret works as a fake HSM's entropy source inside the sample KMS, which is used
/// to derive keys.
pub const ROOT_SECRET: &[u8] = &[
    217, 155, 119, 5, 176, 186, 122, 22, 130, 149, 179, 163, 54, 114, 112, 176, 221, 155, 55, 27,
    245, 20, 202, 139, 155, 167, 240, 163, 55, 17, 218, 234,
];

#[derive(Serialize, Deserialize)]
struct Ciphertext {
    data: Vec<u8>,
    iv: [u8; 12],
}

/// A fake a KMS implementation
pub struct SampleKms {
    /// storage for the generated keys
    keys: HashSet<String>,
    root_secret: Vec<u8>,
}

#[async_trait]
impl KMS for SampleKms {
    async fn generate_key(&mut self) -> Result<String> {
        let keyid = loop {
            let uuid = uuid::Uuid::new_v4().to_string();

            match self.keys.insert(uuid.clone()) {
                true => break uuid,
                false => continue,
            }
        };

        Ok(keyid)
    }

    async fn encrypt(&mut self, data: &[u8], keyid: &str) -> Result<Vec<u8>> {
        if !self.check_key_exist(keyid) {
            bail!("keyid {keyid} not exists inside KMS");
        }

        let key = Zeroizing::new(self.hkdf(keyid.as_bytes())?.into());
        let mut iv = [0u8; 12];

        // Although thread_rng is a CSPRNG, it still does not avoid some flaws.
        // However, for a demo it is enough here. For more information:
        // https://rust-random.github.io/book/guide-rngs.html#not-a-crypto-library
        rand::thread_rng().fill_bytes(&mut iv);
        let data = crypto::encrypt(key, data.into(), iv.into(), WrapType::Aes256Gcm.as_ref())?;
        let cp = Ciphertext { data, iv };

        Ok(serde_json::to_vec(&cp)?)
    }

    async fn decrypt(&mut self, ciphertext: &[u8], keyid: &str) -> Result<Vec<u8>> {
        if !self.check_key_exist(keyid) {
            bail!("keyid {keyid} not exists inside KMS");
        }

        let cp: Ciphertext = serde_json::from_slice(ciphertext)?;
        let key = Zeroizing::new(self.hkdf(keyid.as_bytes())?.into());
        crypto::decrypt(key, cp.data, cp.iv.into(), WrapType::Aes256Gcm.as_ref())
    }
}

impl SampleKms {
    /// Use [HKDF](https://tools.ietf.org/html/rfc5869) to derive a 256-bit
    /// key for AES-256-GCM. The IKM is the secret of the [`SampleKms`]
    fn hkdf(&self, salt: &[u8]) -> Result<[u8; 32]> {
        let hk = Hkdf::<Sha256>::new(Some(salt), &self.root_secret);
        let mut key = [0u8; 32];
        hk.expand(b"key", &mut key).map_err(|e| anyhow!("{}", e))?;
        Ok(key)
    }

    /// Check whether the key of given keyid exists inside the KMS.
    fn check_key_exist(&self, keyid: &str) -> bool {
        self.keys.contains(keyid)
    }
}

impl Default for SampleKms {
    fn default() -> Self {
        Self {
            keys: Default::default(),
            root_secret: ROOT_SECRET.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{plugins::sample::SampleKms, KMS};

    #[rstest]
    #[case(b"this is a test plaintext")]
    #[case(b"this is a another test plaintext")]
    #[tokio::test]
    async fn key_lifetime(#[case] plaintext: &[u8]) {
        let mut kms = SampleKms::default();
        let keyid = kms.generate_key().await.expect("generate key");
        let ciphertext = kms.encrypt(plaintext, &keyid).await.expect("encrypt");
        let decrypted = kms.decrypt(&ciphertext, &keyid).await.expect("decrypt");
        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    async fn encrypt_with_an_non_existent_key() {
        let mut kms = SampleKms::default();
        let ciphertext = kms.encrypt(b"a test text", "an-non-existent-key-id").await;
        assert!(ciphertext.is_err())
    }

    #[tokio::test]
    async fn decrypt_with_an_non_existent_key() {
        let mut kms = SampleKms::default();
        let keyid = kms.generate_key().await.expect("generate key");
        let ciphertext = kms.encrypt(b"a test text", &keyid).await.expect("encrypt");

        // Use a fake key id to decrypt
        let decrypted = kms.decrypt(&ciphertext, "an-non-existent-key-id").await;
        assert!(decrypted.is_err())
    }
}
