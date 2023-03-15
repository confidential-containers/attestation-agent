// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use super::Attester;
use anyhow::*;
use serde::{Deserialize, Serialize};
use sev::firmware::guest::types::AttestationReport;
use sev::firmware::host::types::CertTableEntry;
use tss_esapi::abstraction::nv;
use tss_esapi::interface_types::resource_handles::NvAuth;
use tss_esapi::interface_types::session_handles::AuthSession;
use tss_esapi::tcti_ldr::{DeviceConfig, TctiNameConf};
use tss_esapi::Context;

const SNP_REPORT_SIZE: usize = std::mem::size_of::<AttestationReport>();
const VTPM_NV_INDEX: u32 = 0x01400001;
const VTPM_REPORT_OFFSET: usize = 32;

fn get_attestation_report() -> anyhow::Result<AttestationReport> {
    use tss_esapi::handles::NvIndexTpmHandle;
    let nv_index = NvIndexTpmHandle::new(VTPM_NV_INDEX)?;

    let conf: TctiNameConf = TctiNameConf::Device(DeviceConfig::default());
    let mut context = Context::new(conf)?;
    let auth_session = AuthSession::Password;
    context.set_sessions((Some(auth_session), None, None));

    let bytes = nv::read_full(&mut context, NvAuth::Owner, nv_index)?;
    let report_bytes = bytes[VTPM_REPORT_OFFSET..(VTPM_REPORT_OFFSET + SNP_REPORT_SIZE)].to_vec();
    let attestion_report: AttestationReport = bincode::deserialize(&report_bytes)?;
    Ok(attestion_report)
}

pub fn detect_platform() -> bool {
    let conf: TctiNameConf = TctiNameConf::Device(DeviceConfig::default());
    Context::new(conf).map(|_| true).unwrap_or(false)
}

#[derive(Serialize, Deserialize)]
struct SnpEvidence {
    attestation_report: AttestationReport,
    cert_chain: Vec<CertTableEntry>,
}

#[derive(Debug, Default)]
pub struct VtpmAttester {}

impl Attester for VtpmAttester {
    fn get_evidence(&self, report_data: String) -> Result<String> {
        todo!();
    }
}
