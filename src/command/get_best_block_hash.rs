use crate::client::Client;
/*
getbestblockhash

Returns the hash of the best (tip) block in the most-work fully-validated chain.

Result:
"hex"    (string) the block hash, hex-encoded

Examples:
> bitcoin-cli getbestblockhash
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getbestblockhash", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use crate::BlockhashHexEncoded;
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
