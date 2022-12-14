use crate::client::Client;
/*
getchaintips
Return information about all known tips in the block tree, including the main chain as well as orphaned branches.

Result:
[                        (json array)
  {                      (json object)
    "height" : n,        (numeric) height of the chain tip
    "hash" : "hex",      (string) block hash of the tip
    "branchlen" : n,     (numeric) zero for main chain, otherwise length of branch connecting the tip to the main chain
    "status" : "str"     (string) status of the chain, "active" for the main chain
                         Possible values for status:
                         1.  "invalid"               This branch contains at least one invalid block
                         2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
                         3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
                         4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
                         5.  "active"                This is the tip of the active main chain, which is certainly valid
  },
  ...
]

Examples:
> bitcoin-cli getchaintips
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getchaintips", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use crate::BlockhashHexEncoded;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::{to_raw_value, RawValue};

pub struct GetChainTipsCommand {}
impl GetChainTipsCommand {
    pub fn new() -> Self {
        GetChainTipsCommand {}
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Tip {
    height: u64,    // height of the chain tip
    hash: String,   // "hex" block hash of the tip
    branchlen: u64, // zero for main chain, otherwise length of branch connecting the tip to the main chain
    // TODO: Represent the 5 possible string values this can be using an enum.
    status: String, //status of the chain, "active" for the main chain
                    // Possible values for status:
                    // 1.  "invalid"               This branch contains at least one invalid block
                    // 2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
                    // 3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
                    // 4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
                    // 5.  "active"                This is the tip of the active main chain, which is certainly valid
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChainTipsCommandResponse(Vec<Tip>);

impl CallableCommand for GetChainTipsCommand {
    type Response = GetChainTipsCommandResponse;
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error> {
        let command = "getchaintips";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetChainTipsCommandResponse = r.result()?;
        Ok(response)
    }
}
