// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use anyhow::*;

pub mod sample;

#[cfg(feature = "tdx-attester")]
pub mod tdx;

#[cfg(feature = "sev-snp-vtpm-attester")]
pub mod sev_snp_vtpm;

/// The supported TEE types:
/// - Tdx: TDX TEE.
/// - Sgx: SGX TEE.
/// - Sevsnp: SEV-SNP TEE.
/// - Sample: A dummy TEE that used to test/demo the KBC functionalities.
#[derive(Debug, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum Tee {
    Tdx,
    Sgx,
    Sevsnp,
    SevSnpVtpm,
    Sample,
    Unknown,
}

impl Tee {
    pub fn to_attester(&self) -> Result<Box<dyn Attester + Send + Sync>> {
        match self {
            Tee::Sample => Ok(Box::<sample::SampleAttester>::default()),
            #[cfg(feature = "tdx-attester")]
            Tee::Tdx => Ok(Box::<tdx::TdxAttester>::default()),
            #[cfg(feature = "sev-snp-vtpm-attester")]
            Tee::SevSnpVtpm => Ok(Box::<sev_snp_vtpm::VtpmAttester>::default()),
            _ => bail!("TEE is not supported!"),
        }
    }
}

pub trait Attester {
    fn get_evidence(&self, report_data: String) -> Result<String>;
}

// Detect which TEE platform the KBC running environment is.
pub fn detect_tee_type() -> Tee {
    if sample::detect_platform() {
        return Tee::Sample;
    }
    #[cfg(feature = "tdx-attester")]
    if tdx::detect_platform() {
        return Tee::Tdx;
    }
    #[cfg(feature = "sev-snp-vtpm-attester")]
    if sev_snp_vtpm::detect_platform() {
        return Tee::SevSnpVtpm;
    }
    Tee::Unknown
}
