// Copyright The attestation-agent Authors.
// SPDX-License-Identifier: Apache-2.0

extern crate base64;
extern crate futures;
extern crate grpcio;
extern crate serde;

extern crate lib;

use std::io::Read;
use std::str;
use std::sync::Arc;
use std::{io, thread};

use futures::prelude::*;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use futures::channel::oneshot;
use futures::executor::block_on;
use lib::kbc::*;
use lib::keyprovider;
use lib::keyprovider_grpc::{self, KeyProviderService};
use lib::messages_grpc::*;

#[derive(Clone)]
struct KeyProvider;

impl KeyProviderService for KeyProvider {
    fn un_wrap_key(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: keyprovider::keyProviderKeyWrapProtocolInput,
        sink: ::grpcio::UnarySink<keyprovider::keyProviderKeyWrapProtocolOutput>,
    ) {
        // Deserialize gRPC input to get kbs_annotation_packet
        let input_string = String::from_utf8(req.KeyProviderKeyWrapProtocolInput).unwrap();
        let input: KeyProviderInput =
            serde_json::from_str::<KeyProviderInput>(&input_string[..]).unwrap();
        let base64_annotation = input.keyunwrapparams.annotation.unwrap();
        let vec_annotation = base64::decode(base64_annotation).unwrap();
        let jsonstring_annotation: &str = str::from_utf8(&vec_annotation).unwrap();

        // Initialize kbs client module, and call the trait interface to get decrypted optsdata(plain PLBCO)
        let kbc_module = sample_kbc::SampleKbsClient::new(jsonstring_annotation);
        let decrypt_optsdata = kbc_module.unwrap_kbs_annotation();

        // Construct output structure and serialize it as the return value of gRPC
        let output_struct = KeyUnwrapOutput {
            keyunwrapresults: KeyUnwrapResults {
                optsdata: decrypt_optsdata,
            },
        };
        let mut result = keyprovider::keyProviderKeyWrapProtocolOutput::new();
        result.KeyProviderKeyWrapProtocolOutput = serde_json::to_string(&output_struct)
            .unwrap()
            .as_bytes()
            .to_vec();
        let unwrap_output = sink
            .success(result.clone())
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err))
            .map(move |_| println!("Responded with result {{ {:?} }}", result));
        ctx.spawn(unwrap_output)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = keyprovider_grpc::create_key_provider_service(KeyProvider);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 44444)
        .build()
        .unwrap();
    server.start();
    for (ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}
