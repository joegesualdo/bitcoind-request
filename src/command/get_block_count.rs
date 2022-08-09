/*
getblockcount

Returns the height of the most-work fully-validated chain.
The genesis block has height 0.

Result:
n    (numeric) The current block count

Examples:
> bitcoin-cli getblockcount
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockcount", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::request::request;
use crate::command::CallableCommand;
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

pub struct GetBlockCountCommand {}
impl GetBlockCountCommand {
    pub fn new() -> Self {
        GetBlockCountCommand {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockCountCommandResponse(pub u64);

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
