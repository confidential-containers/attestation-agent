extern crate serde;

use super::KbsClientInterface;

use self::serde::{Deserialize, Serialize};
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use std::error::Error;

// KBS specific packet
#[derive(Serialize, Deserialize, Debug)]
pub struct AnnotationPacket {
    pub key_url: String,
    pub wrapped_key: Vec<u8>,
    pub wrap_type: String,
}

pub struct SampleKbsClient {
    // encrypted PLBCO: get from annotation packet
    encrypted_PLBCO: Vec<u8>,
}

// As a KBS client for attestation-agent,
// you must implement KbsClientInterface trait for it
impl KbsClientInterface for SampleKbsClient {
    fn unwrap_kbs_annotation(&self) -> Vec<u8> {
        self.decrypt_key().unwrap()
    }
}

impl SampleKbsClient {
    pub fn new(jsonstring_annotation: &str) -> SampleKbsClient {
        let annotation_packet: AnnotationPacket =
            serde_json::from_str(jsonstring_annotation).unwrap();
        let encrypted_PLBCO = annotation_packet.wrapped_key;
        SampleKbsClient {
            encrypted_PLBCO: encrypted_PLBCO,
        }
    }
    fn decrypt_key(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let cipher_text: &Vec<u8> = &self.encrypted_PLBCO;
        let decrypting_key = Key::from_slice(b"passphrasewhichneedstobe32bytes!");
        let cipher = Aes256Gcm::new(decrypting_key);
        let nonce = Nonce::from_slice(b"unique nonce");

        let plain_text = cipher
            .decrypt(nonce, cipher_text.as_ref())
            .expect("decryption failure!");

        Ok(plain_text)
    }
}
