use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use crate::BlockhashHexEncoded;
use jsonrpc::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::{to_raw_value, RawValue};

pub struct GetBestBlockHashCommand {
    blockhash_hex_encoded: BlockhashHexEncoded,
}
impl GetBestBlockHashCommand {
    pub fn new(blockhash_hex_encoded: BlockhashHexEncoded) -> Self {
        GetBestBlockHashCommand {
            blockhash_hex_encoded,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetBestBlockHashCommandResponse(pub Blockhash);

impl CallableCommand for GetBestBlockHashCommand {
    type Response = GetBestBlockHashCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let blockhash_arg = &self.blockhash_hex_encoded.0;
        let command = "getbestblockhash";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetBestBlockHashCommandResponse = r.result().unwrap();
        response
    }
}
