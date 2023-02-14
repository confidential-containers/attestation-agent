// Copyright (c) 2023 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use anyhow::*;
use attestation_agent::kbc_modules::cc_kbc::attester::{sgx_occlum::SgxOcclumAttester, Attester};

fn real_main() -> Result<String> {
    let sgx_attester = SgxOcclumAttester {};
    sgx_attester.get_evidence("test".into())
}

fn main() {
    match real_main() {
        std::result::Result::Ok(s) => println!("Get quote successfully: {s}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
