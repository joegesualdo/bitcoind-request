/*
getconnectioncount

Returns the number of connections to other nodes.

Result:
n    (numeric) The connection count

Examples:
> bitcoin-cli getconnectioncount
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getconnectioncount", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::CallableCommand;
use crate::{client::Client, command::request::request};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

pub struct GetConnectionCountCommand {}
impl GetConnectionCountCommand {
    pub fn new() -> Self {
        GetConnectionCountCommand {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetConnectionCountCommandResponse(pub u64);

impl CallableCommand for GetConnectionCountCommand {
    type Response = GetConnectionCountCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let command = "getconnectioncount";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetConnectionCountCommandResponse = r.result().unwrap();
        response
    }
}
