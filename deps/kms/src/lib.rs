// Copyright (c) 2023 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

//! # KMS
//!
//! This lib defines the API of the KMSes which will be used to implement
//! encryption and decryption.
//!
//! ## Trait Definitions
//!
//! ### KMS
//!
//! [`KMS`] defines the interface of KMS.

pub mod api;
pub use api::KMS;

pub mod plugins;

pub mod types;
