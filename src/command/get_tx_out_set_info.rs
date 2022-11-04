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

use crate::client::Client;
use crate::command::request::request;
use crate::command::CallableCommand;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::{to_raw_value, RawValue};

pub struct GetTxOutSetInfoCommand {}
impl GetTxOutSetInfoCommand {
    pub fn new() -> Self {
        GetTxOutSetInfoCommand {}
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTxOutSetInfoCommandResponse {
    pub height: u64,               // The current block height (index)
    pub bestblock: String,         // "hex" The hash of the block at the tip of the chain
    pub transactions: u64,         // The number of transactions with unspent outputs
    pub txouts: u64,               // The number of unspent transaction outputs
    pub bogosize: u64,             // A meaningless metric for UTXO set size
    pub hash_serialized_2: String, // "hex" The serialized hash (only present if 'hash_serialized_2' hash_type is chosen)
    pub disk_size: u64,            // The estimated size of the chainstate on disk
    pub total_amount: f64,         // The total amount
}

impl CallableCommand for GetTxOutSetInfoCommand {
    type Response = GetTxOutSetInfoCommandResponse;
    // TODO: This currently fails. Seems realted to this: https://github.com/bitcoin/bitcoin/issues/25724
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error> {
        let command = GET_TX_OUT_SET_INFO_COMMAND;
        // TODO: Implemnt hashtype arg (wasn't an option in bitcoin core v0.20)
        // let params: Vec<Box<RawValue>> = vec![hash_type_arg_raw_value];
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetTxOutSetInfoCommandResponse = r.result()?;
        Ok(response)
    }
}
