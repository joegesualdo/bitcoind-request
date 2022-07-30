use crate::command::request::request;
use crate::command::CallableCommand;
use crate::BlockhashHexEncoded;
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::{to_raw_value, RawValue};

pub struct GetBlockCountCommand {}
impl GetBlockCountCommand {
    pub fn new() -> Self {
        GetBlockCountCommand {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockCountCommandResponse(u64);

impl CallableCommand for GetBlockCountCommand {
    type Response = GetBlockCountCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let command = "getblockcount";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetBlockCountCommandResponse = r.result().unwrap();
        response
    }
}
