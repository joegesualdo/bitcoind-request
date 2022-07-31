use crate::command::{request::request, CallableCommand};
use crate::Blockhash;
use jsonrpc::Client;
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
        println!("{:?}", params);
        let r = request(client, command, params);
        let response: GetBlockHeaderCommandResponse = r.result().unwrap();
        response
    }
}
