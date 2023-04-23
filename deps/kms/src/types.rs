// Copyright (c) 2023 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use anyhow::Result;
use strum::{AsRefStr, EnumString};

use crate::KMS;

#[derive(EnumString, AsRefStr)]
pub enum KMSTypes {
    #[cfg(feature = "sample")]
    #[strum(serialize = "sample")]
    Sample,
}

impl KMSTypes {
    pub fn to_kms(&self) -> Result<Box<dyn KMS>> {
        match self {
            #[cfg(feature = "sample")]
            KMSTypes::Sample => Ok(Box::<crate::plugins::sample::SampleKms>::default()),
        }
    }
}
