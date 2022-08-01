/*
gettxoutsetinfo ( "hash_type" )

Returns statistics about the unspent transaction output set.
Note this call may take some time.

Arguments:
1. hash_type    (string, optional, default=hash_serialized_2) Which UTXO set hash should be calculated. Options: 'hash_serialized_2' (the legacy algorithm), 'none'.

Result:
{                                 (json object)
  "height" : n,                   (numeric) The current block height (index)
  "bestblock" : "hex",            (string) The hash of the block at the tip of the chain
  "transactions" : n,             (numeric) The number of transactions with unspent outputs
  "txouts" : n,                   (numeric) The number of unspent transaction outputs
  "bogosize" : n,                 (numeric) A meaningless metric for UTXO set size
  "hash_serialized_2" : "hex",    (string) The serialized hash (only present if 'hash_serialized_2' hash_type is chosen)
  "disk_size" : n,                (numeric) The estimated size of the chainstate on disk
  "total_amount" : n              (numeric) The total amount
}

Examples:
> bitcoin-cli gettxoutsetinfo
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "gettxoutsetinfo", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/

const GET_TX_OUT_SET_INFO_COMMAND: &str = "gettxoutsetinfo";
const DEFAULT_HASH_TYPE_ARG: &str = "hash_serialized_2";

use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use crate::BlockhashHexEncoded;
use jsonrpc::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::{to_raw_value, RawValue};

pub struct GetTxOutSetInfoCommand {
    hash_type: String, // (string, optional, default=hash_serialized_2) Which UTXO set hash should be calculated. Options: 'hash_serialized_2' (the legacy algorithm), 'none'.
}
impl GetTxOutSetInfoCommand {
    pub fn new() -> Self {
        GetTxOutSetInfoCommand {
            hash_type: DEFAULT_HASH_TYPE_ARG.to_string(),
        }
    }
    // TODO: hash_type can only be one of two values. Prevent failure case.
    pub fn hash_type(&mut self, hash_type: String) -> &Self {
        self.hash_type = hash_type;
        self
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetTxOutSetInfoCommandResponse {
    height: u64,               // The current block height (index)
    bestblock: String,         // "hex" The hash of the block at the tip of the chain
    transactions: u64,         // The number of transactions with unspent outputs
    txouts: u64,               // The number of unspent transaction outputs
    bogosize: u64,             // A meaningless metric for UTXO set size
    hash_serialized_2: String, // "hex" The serialized hash (only present if 'hash_serialized_2' hash_type is chosen)
    disk_size: u64,            // The estimated size of the chainstate on disk
    total_amount: u64,         // The total amount
}

impl CallableCommand for GetTxOutSetInfoCommand {
    type Response = GetTxOutSetInfoCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let command = GET_TX_OUT_SET_INFO_COMMAND;
        let hash_type_arg = &self.hash_type;
        let hash_type_arg_raw_value = to_raw_value(&hash_type_arg).unwrap();
        let params: Vec<Box<RawValue>> = vec![hash_type_arg_raw_value];
        let r = request(client, command, params);
        let response: GetTxOutSetInfoCommandResponse = r.result().unwrap();
        response
    }
}
