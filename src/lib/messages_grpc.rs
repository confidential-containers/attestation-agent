// Copyright The attestation-agent Authors.
// SPDX-License-Identifier: Apache-2.0

extern crate serde;

use self::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyProviderInput {
    // Operation is either "keywrap" or "keyunwrap"
    // attestation-agent can only handle the case of "keyunwrap" 
    op: String,
    // For attestation-agent, keywrapparams should be empty.
    pub keywrapparams: KeyWrapParams,
    pub keyunwrapparams: KeyUnwrapParams,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyWrapParams {
    // For attestation-agent, ec is null
    pub ec: Option<Ec>,
    // For attestation-agent, optsdata is null
    pub optsdata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ec {
    pub Parameters: HashMap<String, Vec<String>>,
    pub DecryptConfig: Dc,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyUnwrapParams {
    pub dc: Option<Dc>,
    pub annotation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dc {
    pub Parameters: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyUnwrapOutput {
    pub keyunwrapresults: KeyUnwrapResults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyUnwrapResults {
    pub optsdata: Vec<u8>,
}
