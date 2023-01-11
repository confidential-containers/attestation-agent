// Copyright (c) 2023 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use super::*;
use ::ttrpc::Server;
use std::path::Path;

const DEFAULT_KEYPROVIDER_UNIX_ADDR: &str =
    "unix:///opt/confidential-containers/attestation-agent/keyprovider.sock";
const DEFAULT_GETRESOURCE_UNIX_ADDR: &str =
    "unix:///opt/confidential-containers/attestation-agent/getresource.sock";

lazy_static! {
    pub static ref SYNC_ATTESTATION_AGENT: Arc<std::sync::Mutex<AttestationAgent>> =
        Arc::new(std::sync::Mutex::new(AttestationAgent::new()));
}

pub fn ttrpc_main() {
    let app_matches = App::new(rpc::AGENT_NAME)
            .version(env!("CARGO_PKG_VERSION"))
            .arg(
                Arg::with_name("KeyProvider ttRPC Unix socket addr")
                    .long("keyprovider_sock")
                    .takes_value(true)
                    .help("This Unix socket address which the KeyProvider ttRPC service will listen to, for example: --keyprovider_sock unix:///tmp/aa_keyprovider",
                    ),
            )
            .arg(
                Arg::with_name("GetResource ttRPC Unix socket addr")
                    .long("getresource_sock")
                    .takes_value(true)
                    .help("This Unix socket address which the GetResource ttRPC service will listen to, for example: --getresource_sock unix:///tmp/aa_getresource",
                    ),
            )
            .get_matches();

    let keyprovider_socket = app_matches
        .value_of("KeyProvider ttRPC Unix socket addr")
        .unwrap_or(DEFAULT_KEYPROVIDER_UNIX_ADDR);

    let getresource_socket = app_matches
        .value_of("GetResource ttRPC Unix socket addr")
        .unwrap_or(DEFAULT_GETRESOURCE_UNIX_ADDR);

    debug!(
        "KeyProvider ttRPC service listening on: {:?}",
        keyprovider_socket
    );
    debug!(
        "GetResource ttRPC service listening on: {:?}",
        getresource_socket
    );

    clean_previous_sock_file(&keyprovider_socket).unwrap();
    clean_previous_sock_file(&getresource_socket).unwrap();

    let keyprovider_service = rpc::keyprovider::ttrpc::ttrpc_service();
    let getresource_service = rpc::getresource::ttrpc::ttrpc_service();

    let mut keyprovider_server = Server::new()
        .bind(&keyprovider_socket)
        .unwrap()
        .register_service(keyprovider_service);
    keyprovider_server.start().unwrap();

    let mut getresource_server = Server::new()
        .bind(&getresource_socket)
        .unwrap()
        .register_service(getresource_service);
    getresource_server.start().unwrap();
}

fn clean_previous_sock_file(unix_socket: &str) -> Result<()> {
    let path = unix_socket
        .strip_prefix("unix://")
        .expect("socket address scheme is not expected");

    if Path::new(path).exists() {
        std::fs::remove_file(&path)?;
    }

    Ok(())
}
