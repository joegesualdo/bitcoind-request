/*
getblockhash height

Returns hash of block in best-block-chain at height provided.

Arguments:
1. height    (numeric, required) The height index

Result:
"hex"    (string) The block hash

Examples:
> bitcoin-cli getblockhash 1000
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockhash", "params": [1000]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
 */
use crate::command::CallableCommand;
use crate::Blockhash;
use crate::{client::Client, command::request::request};
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
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error> {
        let height_arg = &self.height;
        let height_arg_raw_value = to_raw_value(height_arg).unwrap();
        let command = "getblockhash";
        let params = vec![height_arg_raw_value];
        let r = request(client, command, params);
        let response: GetBlockHashCommandResponse = r.result()?;
        Ok(response)
    }
}
