// use jsonrpc::{Client, Request, Response};
use crate::client::Client;
use jsonrpc::Response;
use serde_json::value::RawValue;

// Note: Callers must convert their paramaters to a serde_json Raw value:
//    let blockhash = "839832983298"
//    let blockhash_arg_raw_value = to_raw_value(&blockhash).unwrap();
//    let params = vec![blockhash_arg_raw_value]
pub fn request(client: &Client, command: &str, params: Vec<Box<RawValue>>) -> Response {
    let request = client.build_request(command, &params);
    let error_message = format!("{}_failed", command);
    let response = client.send_request(request).expect(&error_message);
    response
}
