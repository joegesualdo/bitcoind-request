/*
getmininginfo

Returns a json object containing mining-related information.
Result:
{                              (json object)
  "blocks" : n,                (numeric) The current block
  "currentblockweight" : n,    (numeric, optional) The block weight of the last assembled block (only present if a block was ever assembled)
  "currentblocktx" : n,        (numeric, optional) The number of block transactions of the last assembled block (only present if a block was ever assembled)
  "difficulty" : n,            (numeric) The current difficulty
  "networkhashps" : n,         (numeric) The network hashes per second
  "pooledtx" : n,              (numeric) The size of the mempool
  "chain" : "str",             (string) current network name (main, test, regtest)
  "warnings" : "str"           (string) any network and blockchain warnings
}

Examples:
> bitcoin-cli getmininginfo
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getmininginfo", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::{to_raw_value, RawValue};

const GET_DIFFICULTY_COMMAND: &str = "getmininginfo";

pub struct GetMiningInfoCommand {}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetMiningInfoCommandResponse {
    pub blocks: u64,                     // The current block
    pub currentblockweight: Option<u64>, //  The block weight of the last assembled block (only present if a block was ever assembled)
    pub currentblocktx: Option<u64>, //  The number of block transactions of the last assembled block (only present if a block was ever assembled)
    pub difficulty: f64,             // The current difficulty
    pub networkhashps: f64,          // The network hashes per second
    pub pooledtx: u64,               // The size of the mempool
    pub chain: String,               // current network name (main, test, regtest)
    pub warnings: String,            // any network and blockchain warnings
}
impl GetMiningInfoCommand {
    pub fn new() -> Self {
        GetMiningInfoCommand {}
    }
}

impl CallableCommand for GetMiningInfoCommand {
    type Response = GetMiningInfoCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let params = vec![];
        let r = request(client, GET_DIFFICULTY_COMMAND, params);
        let response: GetMiningInfoCommandResponse = r.result().unwrap();
        response
    }
}
