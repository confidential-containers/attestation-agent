// Copyright (c) 2023 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait KMS {
    /// Generate a key inside the KMS, and return an unique id string
    /// that represents the key inside KMS. Then this unique id string
    /// can be used to encrypt data slice or decrypt ciphertext that
    /// has been encrypted with the key.
    ///
    /// Note: the returned string can only allow letters, numbers,
    /// underscores `_` and hyphen `-` are allowed
    async fn generate_key(&mut self) -> Result<String>;

    /// Use the key of `keyid` to encrypt the `data` slice inside KMS, and then
    /// return the ciphertext of the `data`. The encryption operation should occur
    /// inside KMS. This function only works as a wrapper for different KMS APIs
    async fn encrypt(&mut self, data: &[u8], keyid: &str) -> Result<Vec<u8>>;

    /// Use the key of `keyid` to decrypt the `ciphertext` slice inside KMS, and then
    /// return the plaintext of the `data`. The decryption operation should occur
    /// inside KMS. This function only works as a wrapper for different KMS APIs
    async fn decrypt(&mut self, ciphertext: &[u8], keyid: &str) -> Result<Vec<u8>>;
}
