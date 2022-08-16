/*
getdifficulty

Returns the proof-of-work difficulty as a multiple of the minimum difficulty.

Result:
n    (numeric) the proof-of-work difficulty as a multiple of the minimum difficulty.

Examples:
> bitcoin-cli getdifficulty
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getdifficulty", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::CallableCommand;
use crate::Blockhash;
use crate::{client::Client, command::request::request};
use serde::{Deserialize, Serialize};
use serde_json::value::{to_raw_value, RawValue};

const GET_DIFFICULTY_COMMAND: &str = "getdifficulty";

pub struct GetDifficultyCommand {}
impl GetDifficultyCommand {
    pub fn new() -> Self {
        GetDifficultyCommand {}
    }
}
// TODO: struct GetDifficultyCommandResponse(String);
#[derive(Serialize, Deserialize, Debug)]
pub struct GetDifficultyCommandResponse(pub f64);

impl CallableCommand for GetDifficultyCommand {
    type Response = GetDifficultyCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let params = vec![];
        let r = request(client, GET_DIFFICULTY_COMMAND, params);
        let response: GetDifficultyCommandResponse = r.result().unwrap();
        response
    }
}
