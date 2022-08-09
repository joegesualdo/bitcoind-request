/*
getblockchaininfo

Returns an object containing various state info regarding blockchain processing.

Result:
{                                         (json object)
  "chain" : "str",                        (string) current network name (main, test, regtest)
  "blocks" : n,                           (numeric) the height of the most-work fully-validated chain. The genesis block has height 0
  "headers" : n,                          (numeric) the current number of headers we have validated
  "bestblockhash" : "str",                (string) the hash of the currently best block
  "difficulty" : n,                       (numeric) the current difficulty
  "mediantime" : n,                       (numeric) median time for the current best block
  "verificationprogress" : n,             (numeric) estimate of verification progress [0..1]
  "initialblockdownload" : true|false,    (boolean) (debug information) estimate of whether this node is in Initial Block Download mode
  "chainwork" : "hex",                    (string) total amount of work in active chain, in hexadecimal
  "size_on_disk" : n,                     (numeric) the estimated size of the block and undo files on disk
  "pruned" : true|false,                  (boolean) if the blocks are subject to pruning
  "pruneheight" : n,                      (numeric) lowest-height complete block stored (only present if pruning is enabled)
  "automatic_pruning" : true|false,       (boolean) whether automatic pruning is enabled (only present if pruning is enabled)
  "prune_target_size" : n,                (numeric) the target size used by pruning (only present if automatic pruning is enabled)
  "softforks" : {                         (json object) status of softforks
    "xxxx" : {                            (json object) name of the softfork
      "type" : "str",                     (string) one of "buried", "bip9"
      "bip9" : {                          (json object) status of bip9 softforks (only for "bip9" type)
        "status" : "str",                 (string) one of "defined", "started", "locked_in", "active", "failed"
        "bit" : n,                        (numeric) the bit (0-28) in the block version field used to signal this softfork (only for "started" status)
        "start_time" : xxx,               (numeric) the minimum median time past of a block at which the bit gains its meaning
        "timeout" : xxx,                  (numeric) the median time past of a block at which the deployment is considered failed if not yet locked in
        "since" : n,                      (numeric) height of the first block to which the status applies
        "statistics" : {                  (json object) numeric statistics about BIP9 signalling for a softfork (only for "started" status)
          "period" : n,                   (numeric) the length in blocks of the BIP9 signalling period
          "threshold" : n,                (numeric) the number of blocks with the version bit set required to activate the feature
          "elapsed" : n,                  (numeric) the number of blocks elapsed since the beginning of the current period
          "count" : n,                    (numeric) the number of blocks with the version bit set in the current period
          "possible" : true|false         (boolean) returns false if there are not enough blocks left in this period to pass activation threshold
        }
      },
      "height" : n,                       (numeric) height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
      "active" : true|false               (boolean) true if the rules are enforced for the mempool and the next block
    },
    ...
  },
  "warnings" : "str"                      (string) any network and blockchain warnings
}

Examples:
> bitcoin-cli getblockchaininfo
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockchaininfo", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use std::collections::HashMap;

use crate::command::request::request;
use crate::command::CallableCommand;
use jsonrpc::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::RawValue;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub period: u64,    // the length in blocks of the BIP9 signalling period
    pub threshold: u64, // the number of blocks with the version bit set required to activate the feature
    pub elapsed: u64,   // the number of blocks elapsed since the beginning of the current period
    pub count: u64,     //  the number of blocks with the version bit set in the current period
    pub possible: bool, // returns false if there are not enough blocks left in this period to pass activation threshold
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bip9 {
    // TODO: can only be a couple strings
    pub status: String, // one of "defined", "started", "locked_in", "active", "failed"
    pub bit: u64, // the bit (0-28) in the block version field used to signal this softfork (only for "started" status)
    pub start_time: u64, //the minimum median time past of a block at which the bit gains its meaning
    pub timeout: u64, // the median time past of a block at which the deployment is considered failed if not yet locked in
    pub since: u64,   // height of the first block to which the status applies
    pub statistics: Statistics, // numeric statistics about BIP9 signalling for a softfork (only for "started" status)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SoftforkBip9Response {
    #[serde(rename = "type")]
    pub type_: String, // one of "buried", "bip9"
    pub bip9: Bip9,   // status of bip9 softforks (only for "bip9" type)
    pub height: u64, // height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
    pub active: bool, // true if the rules are enforced for the mempool and the next block
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum SoftFork {
    Bip9(SoftforkBip9Response),
    NonBip9(NonBip9SoftforkResponse),
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NonBip9SoftforkResponse {
    #[serde(rename = "type")]
    pub type_: String, // one of "buried", "bip9"
    pub height: u64, // height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status)
    pub active: bool, // true if the rules are enforced for the mempool and the next block
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockchainInfoCommandResponse {
    pub chain: String,         // current network name (main, test, regtest)
    pub blocks: u64, //the height of the most-work fully-validated chain. The genesis block has height 0
    pub headers: u64, // the current number of headers we have validated
    pub bestblockhash: String, //the hash of the currently best block
    pub difficulty: f64, //  the current difficulty
    pub mediantime: u64, //  median time for the current best block
    //  TODO: is only between 0-1
    pub verificationprogress: f64, //  estimate of verification progress [0..1]
    pub initialblockdownload: bool, // (debug information) estimate of whether this node is in Initial Block Download mode
    pub chainwork: String,          // "hex" total amount of work in active chain, in hexadecimal
    pub size_on_disk: u64,          // the estimated size of the block and undo files on disk
    pub pruned: bool,               // if the blocks are subject to pruning
    pub pruneheight: Option<u64>, // lowest-height complete block stored (only present if pruning is enabled)
    pub automatic_pruning: Option<bool>, // whether automatic pruning is enabled (only present if pruning is enabled)
    pub prune_target_size: Option<u64>, //the target size used by pruning (only present if automatic pruning is enabled)
    pub softforks: HashMap<String, SoftFork>, // status of softforks
    pub warnings: String,               // any network and blockchain warnings
}

pub struct GetBlockchainInfoCommand {}

impl GetBlockchainInfoCommand {
    pub fn new() -> Self {
        GetBlockchainInfoCommand {}
    }
}

impl CallableCommand for GetBlockchainInfoCommand {
    type Response = GetBlockchainInfoCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let command = "getblockchaininfo";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetBlockchainInfoCommandResponse = r.result().unwrap();
        response
    }
}
