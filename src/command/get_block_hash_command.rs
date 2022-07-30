use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::{to_raw_value, RawValue};

type BlockHeight = u64;
pub struct GetBlockHashCommand {
    height: BlockHeight,
}
impl GetBlockHashCommand {
    pub fn new(height: BlockHeight) -> Self {
        GetBlockHashCommand { height }
    }
}
// TODO: struct GetBlockHashCommandResponse(String);
#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockHashCommandResponse(pub Blockhash);

impl CallableCommand for GetBlockHashCommand {
    type Response = GetBlockHashCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let height_arg = &self.height;
        let height_arg_raw_value = to_raw_value(height_arg).unwrap();
        let command = "getblockhash";
        let params = vec![height_arg_raw_value];
        let r = request(client, command, params);
        let response: GetBlockHashCommandResponse = r.result().unwrap();
        response
    }
}
