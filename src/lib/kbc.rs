// Copyright The attestation-agent Authors.
// SPDX-License-Identifier: Apache-2.0

pub mod sample_kbc;
// Add your specific kbs client declaration here.
// For example: "mod your_client;"

pub trait KbsClientInterface {
    fn unwrap_kbs_annotation(&self) -> Vec<u8>;
}
