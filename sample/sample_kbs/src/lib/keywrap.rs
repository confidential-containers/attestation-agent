use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use std::error::Error;

pub fn encrypt_key(plain_text: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let encrypting_key = Key::from_slice(b"passphrasewhichneedstobe32bytes!");
    let cipher = Aes256Gcm::new(encrypting_key);
    let nonce = Nonce::from_slice(b"unique nonce");

    let cipher_text = cipher
        .encrypt(nonce, plain_text.as_ref())
        .expect("encryption failure!");

    Ok(cipher_text)
}

pub fn decrypt_key(cipher_text: &Vec<u8>) -> Result<Vec<u8>, Box<dyn Error>> {
    let decrypting_key = Key::from_slice(b"passphrasewhichneedstobe32bytes!");
    let cipher = Aes256Gcm::new(decrypting_key);
    let nonce = Nonce::from_slice(b"unique nonce");

    let plain_text = cipher
        .decrypt(nonce, cipher_text.as_ref())
        .expect("decryption failure!");

    Ok(plain_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_and_decrypt() {
        let plain_text: Vec<u8> = b"plaintext message".to_vec();
        let cipher_text: Vec<u8> = encrypt_key(&plain_text).unwrap();
        let plain_text_decrypt: Vec<u8> = decrypt_key(&cipher_text).unwrap();

        assert_eq!(plain_text, plain_text_decrypt);
    }
}
