// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

#[macro_use]
extern crate strum;

use anyhow::*;

pub mod sample;

#[cfg(feature = "az-snp-vtpm-attester")]
pub mod az_snp_vtpm;
#[cfg(feature = "cca-attester")]
pub mod cca;
#[cfg(feature = "occlum-attester")]
pub mod sgx_occlum;
#[cfg(feature = "tdx-attester")]
pub mod tdx;

/// The supported TEE types:
/// - Tdx: TDX TEE.
/// - SgxOcclum: SGX TEE with Occlum Libos.
/// - AzSnpVtpm: SEV-SNP TEE for Azure CVMs.
/// - Sevsnp: SEV-SNP TEE.
/// - Sample: A dummy TEE that used to test/demo the KBC functionalities.
/// - Cca: Arm Confidential Compute Architecture TEE.
#[derive(Debug, EnumString, Display)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum Tee {
    Tdx,
    #[strum(serialize = "sgx")]
    SgxOcclum,
    Sevsnp,
    AzSnpVtpm,
    Sample,
    Cca,
    Unknown,
}

impl Tee {
    pub fn to_attester(&self) -> Result<Box<dyn Attester + Send + Sync>> {
        match self {
            Tee::Sample => Ok(Box::<sample::SampleAttester>::default()),
            #[cfg(feature = "cca-attester")]
            Tee::Cca => Ok(Box::<cca::CCAAttester>::default()),
            #[cfg(feature = "tdx-attester")]
            Tee::Tdx => Ok(Box::<tdx::TdxAttester>::default()),
            #[cfg(feature = "occlum-attester")]
            Tee::SgxOcclum => Ok(Box::<sgx_occlum::SgxOcclumAttester>::default()),
            #[cfg(feature = "az-snp-vtpm-attester")]
            Tee::AzSnpVtpm => Ok(Box::<az_snp_vtpm::AzSnpVtpmAttester>::default()),
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

    #[cfg(feature = "occlum-attester")]
    if sgx_occlum::detect_platform() {
        return Tee::SgxOcclum;
    }

    #[cfg(feature = "az-snp-vtpm-attester")]
    if az_snp_vtpm::detect_platform() {
        return Tee::AzSnpVtpm;
    }

    #[cfg(feature = "cca-attester")]
    if cca::detect_platform() {
        return Tee::Cca;
    }

    Tee::Unknown
}
