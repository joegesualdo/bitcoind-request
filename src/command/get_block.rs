/*
getblock "blockhash" ( verbosity )

If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
If verbosity is 1, returns an Object with information about block <hash>.
If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.

Arguments:
1. blockhash    (string, required) The block hash
2. verbosity    (numeric, optional, default=1) 0 for hex-encoded data, 1 for a json object, and 2 for json object with transaction data

Result (for verbosity = 0):
"hex"    (string) A string that is serialized, hex-encoded data for block 'hash'

Result (for verbosity = 1):
{                                 (json object)
  "hash" : "hex",                 (string) the block hash (same as provided)
  "confirmations" : n,            (numeric) The number of confirmations, or -1 if the block is not on the main chain
  "size" : n,                     (numeric) The block size
  "strippedsize" : n,             (numeric) The block size excluding witness data
  "weight" : n,                   (numeric) The block weight as defined in BIP 141
  "height" : n,                   (numeric) The block height or index
  "version" : n,                  (numeric) The block version
  "versionHex" : "hex",           (string) The block version formatted in hexadecimal
  "merkleroot" : "hex",           (string) The merkle root
  "tx" : [                        (json array) The transaction ids
    "hex",                        (string) The transaction id
    ...
  ],
  "time" : xxx,                   (numeric) The block time expressed in UNIX epoch time
  "mediantime" : xxx,             (numeric) The median block time expressed in UNIX epoch time
  "nonce" : n,                    (numeric) The nonce
  "bits" : "hex",                 (string) The bits
  "difficulty" : n,               (numeric) The difficulty
  "chainwork" : "hex",            (string) Expected number of hashes required to produce the chain up to this block (in hex)
  "nTx" : n,                      (numeric) The number of transactions in the block
  "previousblockhash" : "hex",    (string) The hash of the previous block
  "nextblockhash" : "hex"         (string) The hash of the next block
}

Result (for verbosity = 2):
{             (json object)
  ...,        Same output as verbosity = 1
  "tx" : [    (json array)
    {         (json object)
      ...     The transactions in the format of the getrawtransaction RPC. Different from verbosity = 1 "tx" result
    },
    ...
  ]
}

Examples:
> bitcoin-cli getblock "00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09"
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblock", "params": ["00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09"]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::{request::request, CallableCommand};
use crate::Blockhash;
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Vin {
    // TODO: Most vins don't have a coinbase key, so how can I make Vin types based on this?
    pub coinbase: Option<String>,
    // TODO: Why wouldn't a vin have this?
    pub txinwitness: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptPubKey {
    pub asm: String,
    pub hex: String,
    pub address: Option<String>,
    // TODO: Can't use "type" as a key because it's a reserved word in Rust.
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: f64,
    pub n: i64,
    pub script_pub_key: ScriptPubKey,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DecodeRawTransactionResponse {
    pub txid: String,  // "hex" The transaction id
    pub hash: String,  // "hex" The transaction hash (differs from txid for witness transactions)
    pub size: u64,     // The transaction size
    pub vsize: u64,    // The virtual transaction size (differs from size for witness transactions)
    pub version: u64,  //The version
    pub weight: u64,   // The transaction's weight (between vsize*4 - 3 and vsize*4)
    pub locktime: u64, //The lock time
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GetBlockCommandTransactionResponse {
    Raw(DecodeRawTransactionResponse),
    Id(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GetBlockCommandResponse {
    BlockHash(String),
    Block(Block),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub hash: String,        // "hex" (string) the block hash (same as provided)
    pub confirmations: i64, // The number of confirmations, or -1 if the block is not on the main chain
    pub size: u64,          // The block size
    pub strippedsize: u64,  // The block size excluding witness data
    pub weight: u64,        // The block weight as defined in BIP 141
    pub height: u64,        // The block height or index
    pub version: u64,       // (numeric) The block version
    pub version_hex: String, // "hex" The block version formatted in hexadecimal
    pub merkleroot: String, // "hex" The merkle root
    pub tx: Vec<GetBlockCommandTransactionResponse>, // "hex" The transaction ids
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

type GetBlockAsSerializedHextEncodedDataCommandResponse = String;

pub enum GetBlockCommandVerbosity {
    SerializedHexEncodedData,                 // argument of 0
    BlockObjectWithoutTransactionInformation, // argument of 1
    BlockObjectWithTransactionInformation,    // argument of 2
}

pub struct GetBlockCommand {
    blockhash: Blockhash,
    verbosity: GetBlockCommandVerbosity,
}
impl GetBlockCommand {
    pub fn new(blockhash: Blockhash) -> Self {
        GetBlockCommand {
            blockhash,
            verbosity: GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation,
        }
    }
    pub fn verbosity(&mut self, verbosity: GetBlockCommandVerbosity) -> &Self {
        self.verbosity = verbosity;
        self
    }
}

// TODO: This will only work for GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation
//       because the json response has a different structure it returns for each verbosity option.
//       For example, GetBlockCommandVerbosity::BlockObjectWithTransactionInformation will return
//       an array for 'tx' field with full transaction structure, instead of only hashes for the
//       transaction. To accomplish this, we need to figure out how to have serde handle
//       conditional responses and map them to appropriate structs.
impl CallableCommand for GetBlockCommand {
    type Response = GetBlockCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let verbosity_arg = match self.verbosity {
            GetBlockCommandVerbosity::SerializedHexEncodedData => 0,
            GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation => 1,
            GetBlockCommandVerbosity::BlockObjectWithTransactionInformation => 2,
        };
        let blockhash_arg = &self.blockhash.0;
        let blockhash_arg_raw_value = to_raw_value(&blockhash_arg).unwrap();
        let verbosity_arg_raw_value = to_raw_value(&verbosity_arg).unwrap();
        let command = "getblock";
        let params = vec![blockhash_arg_raw_value, verbosity_arg_raw_value];
        println!("{:?}", params);
        let r = request(client, command, params);
        let response: GetBlockCommandResponse = r.result().unwrap();
        response
    }
}
