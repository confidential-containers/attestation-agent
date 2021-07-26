// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KEY_PROVIDER_SERVICE_UN_WRAP_KEY: ::grpcio::Method<super::keyprovider::keyProviderKeyWrapProtocolInput, super::keyprovider::keyProviderKeyWrapProtocolOutput> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/keyprovider.KeyProviderService/UnWrapKey",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct KeyProviderServiceClient {
    client: ::grpcio::Client,
}

impl KeyProviderServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KeyProviderServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn un_wrap_key_opt(&self, req: &super::keyprovider::keyProviderKeyWrapProtocolInput, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keyprovider::keyProviderKeyWrapProtocolOutput> {
        self.client.unary_call(&METHOD_KEY_PROVIDER_SERVICE_UN_WRAP_KEY, req, opt)
    }

    pub fn un_wrap_key(&self, req: &super::keyprovider::keyProviderKeyWrapProtocolInput) -> ::grpcio::Result<super::keyprovider::keyProviderKeyWrapProtocolOutput> {
        self.un_wrap_key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn un_wrap_key_async_opt(&self, req: &super::keyprovider::keyProviderKeyWrapProtocolInput, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keyprovider::keyProviderKeyWrapProtocolOutput>> {
        self.client.unary_call_async(&METHOD_KEY_PROVIDER_SERVICE_UN_WRAP_KEY, req, opt)
    }

    pub fn un_wrap_key_async(&self, req: &super::keyprovider::keyProviderKeyWrapProtocolInput) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keyprovider::keyProviderKeyWrapProtocolOutput>> {
        self.un_wrap_key_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait KeyProviderService {
    fn un_wrap_key(&mut self, ctx: ::grpcio::RpcContext, req: super::keyprovider::keyProviderKeyWrapProtocolInput, sink: ::grpcio::UnarySink<super::keyprovider::keyProviderKeyWrapProtocolOutput>);
}

pub fn create_key_provider_service<S: KeyProviderService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_KEY_PROVIDER_SERVICE_UN_WRAP_KEY, move |ctx, req, resp| {
        instance.un_wrap_key(ctx, req, resp)
    });
    builder.build()
}
