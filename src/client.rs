use std::time::Duration;

use jsonrpc::{
    simple_http::{self, SimpleHttpTransport},
    Client as JsonRPCClient, Request as JsonRPCRequest, Response as JsonRPCResponse,
};
use serde_json::value::RawValue;

pub struct Client {
    json_rpc_client: JsonRPCClient,
}

pub struct Request<'a>(JsonRPCRequest<'a>);

impl<'a> Request<'a> {}

impl Client {
    // TODO: Add error handling if this fails
    pub fn new(url: &str, user: &str, pass: &str) -> Result<Self, simple_http::Error> {
        // The default in the library is 15 seconds, but we're setting to very high here to prevent error
        // during the call to gettxoutsetinfo.
        let timeout = Duration::from_secs(300);
        let simple_http_transport = SimpleHttpTransport::builder()
            .url(url)?
            .auth(user, Some(pass))
            .timeout(timeout)
            .build();
        let client = Client {
            json_rpc_client: JsonRPCClient::with_transport(simple_http_transport),
        };
        Ok(client)
    }
    pub fn build_request<'a>(
        &self,
        command: &'a str,
        params: &'a Vec<Box<RawValue>>,
    ) -> Request<'a> {
        let json_rpc_request = self.json_rpc_client.build_request(command, &params);
        let request = Request(json_rpc_request);
        request
    }
    pub fn send_request(&self, request: Request) -> Result<JsonRPCResponse, jsonrpc::Error> {
        let response = self.json_rpc_client.send_request(request.0);
        response
    }
}
