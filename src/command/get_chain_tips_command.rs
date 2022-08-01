use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use crate::BlockhashHexEncoded;
use jsonrpc::Client;
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
    fn call(&self, client: &Client) -> Self::Response {
        let command = "getchaintips";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetChainTipsCommandResponse = r.result().unwrap();
        response
    }
}
