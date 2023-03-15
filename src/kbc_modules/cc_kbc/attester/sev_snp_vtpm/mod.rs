// Copyright (c) 2022 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use super::Attester;
use anyhow::*;
use serde::{Deserialize, Serialize};
use vtpm_snp::hcl::HclReportWithRuntimeData;
use vtpm_snp::vtpm;

pub fn detect_platform() -> bool {
    vtpm::has_tpm_device()
}

#[derive(Debug, Default)]
pub struct VtpmAttester;

#[derive(Serialize, Deserialize)]
struct VtpmSnpEvidence {
    quote: vtpm::Quote,
    hcl_report: HclReportWithRuntimeData,
}

impl Attester for VtpmAttester {
    fn get_evidence(&self, report_data: String) -> Result<String> {
        let hcl_report = vtpm::get_report()?;
        let quote = vtpm::get_quote(&report_data.as_bytes())?;

        let evidence = VtpmSnpEvidence { quote, hcl_report };

        Ok(serde_json::to_string(&evidence)?)
    }
}
