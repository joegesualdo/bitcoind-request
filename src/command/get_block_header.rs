use crate::client::Client;
/*
getblockheader "blockhash" ( verbose )

If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
If verbose is true, returns an Object with information about blockheader <hash>.

Arguments:
1. blockhash    (string, required) The block hash
2. verbose      (boolean, optional, default=true) true for a json object, false for the hex-encoded data

Result (for verbose = true):
{                                 (json object)
  "hash" : "hex",                 (string) the block hash (same as provided)
  "confirmations" : n,            (numeric) The number of confirmations, or -1 if the block is not on the main chain
  "height" : n,                   (numeric) The block height or index
  "version" : n,                  (numeric) The block version
  "versionHex" : "hex",           (string) The block version formatted in hexadecimal
  "merkleroot" : "hex",           (string) The merkle root
  "time" : xxx,                   (numeric) The block time expressed in UNIX epoch time
  "mediantime" : xxx,             (numeric) The median block time expressed in UNIX epoch time
  "nonce" : n,                    (numeric) The nonce
  "bits" : "hex",                 (string) The bits
  "difficulty" : n,               (numeric) The difficulty
  "chainwork" : "hex",            (string) Expected number of hashes required to produce the current chain
  "nTx" : n,                      (numeric) The number of transactions in the block
  "previousblockhash" : "hex",    (string) The hash of the previous block
  "nextblockhash" : "hex"         (string) The hash of the next block
}

Result (for verbose=false):
"hex"    (string) A string that is serialized, hex-encoded data for block 'hash'

Examples:
> bitcoin-cli getblockheader "00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09"
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockheader", "params": ["00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09"]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
 */
use crate::command::{request::request, CallableCommand};
use crate::Blockhash;
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GetBlockHeaderCommandResponse {
    BlockHash(String),
    BlockHeader(BlockHeader),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub hash: String,        // "hex" (string) the block hash (same as provided)
    pub confirmations: i64, // The number of confirmations, or -1 if the block is not on the main chain
    pub height: u64,        // The block height or index
    pub version: u64,       // (numeric) The block version
    pub version_hex: String, // "hex" The block version formatted in hexadecimal
    pub merkleroot: String, // "hex" The merkle root
    pub time: u64,          // "unix epoch time" The block time expressed in UNIX epoch time
    pub mediantime: u64,    // "unix epoch time" The median block time expressed in UNIX epoch time
    pub nonce: u64,         // The nonce
    pub bits: String,       // "hex" The bits
    pub difficulty: f64,    // The difficulty
    pub chainwork: String, // "hex" Expected number of hashes required to produce the chain up to this block (in hex)
    pub n_tx: u64,         // The number of transactions in the block
    pub previousblockhash: Option<String>, // The hash of the previous block
    // TODO: Why isn't this always there?
    pub nextblockhash: Option<String>, // The hash of the next block
}

pub struct GetBlockHeaderCommand {
    blockhash: Blockhash,
    verbose: bool,
}
impl GetBlockHeaderCommand {
    pub fn new(blockhash: Blockhash) -> Self {
        GetBlockHeaderCommand {
            blockhash,
            verbose: true,
        }
    }
    pub fn verbose(&mut self, verbose: bool) -> &Self {
        self.verbose = verbose;
        self
    }
}
impl CallableCommand for GetBlockHeaderCommand {
    type Response = GetBlockHeaderCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let verbose_arg = self.verbose;
        let blockhash_arg = &self.blockhash.0;
        let blockhash_arg_raw_value = to_raw_value(&blockhash_arg).unwrap();
        let verbose_arg_raw_value = to_raw_value(&verbose_arg).unwrap();
        let command = "getblockheader";
        let params = vec![blockhash_arg_raw_value, verbose_arg_raw_value];
        let r = request(client, command, params);
        let response: GetBlockHeaderCommandResponse = r.result().unwrap();
        response
    }
}
