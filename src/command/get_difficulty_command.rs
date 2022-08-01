use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use jsonrpc::Client;
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
