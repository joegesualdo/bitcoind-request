/*
gettxout "txid" n ( include_mempool )

Returns details about an unspent transaction output.

Arguments:
1. txid               (string, required) The transaction id
2. n                  (numeric, required) vout number
3. include_mempool    (boolean, optional, default=true) Whether to include the mempool. Note that an unspent output that is spent in the mempool won't appear.

Result:
{                             (json object)
  "bestblock" : "hex",        (string) The hash of the block at the tip of the chain
  "confirmations" : n,        (numeric) The number of confirmations
  "value" : n,                (numeric) The transaction value in BTC
  "scriptPubKey" : {          (json object)
    "asm" : "hex",            (string)
    "hex" : "hex",            (string)
    "reqSigs" : n,            (numeric) Number of required signatures
    "type" : "hex",           (string) The type, eg pubkeyhash
    "addresses" : [           (json array) array of bitcoin addresses
      "str",                  (string) bitcoin address
      ...
    ]
  },
  "coinbase" : true|false     (boolean) Coinbase or not
}

Examples:

Get unspent transactions
> bitcoin-cli listunspent

View the details
> bitcoin-cli gettxout "txid" 1

As a JSON-RPC call
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "gettxout", "params": ["txid", 1]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
 */

use crate::{
    client::Client,
    command::{request::request, CallableCommand},
};
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;

const GET_TX_OUT_COMMAND: &str = "gettxout";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPubKey {
    pub asm: String,           // "hex"
    pub hex: String,           // "hex"
    pub req_sigs: Option<u64>, // Number of required signatures
    #[serde(alias = "type")]
    pub type_: String, // The type, eg pubkeyhash
    // TODO: Why are there both of these. The docs say there is an "addresses" field
    // (https://bitcoincore.org/en/doc/0.21.0/rpc/blockchain/gettxout/) but the transaction I'm
    // testing only has an "address" field. Why? Will it return either/or?
    pub addresses: Option<Vec<String>>, // array of bitcoin addresses
    pub address: Option<String>,        // bitcoin addresses
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetTxOutCommandResponse {
    bestblock: String,  // "hex" The hash of the block at the tip of the chain
    confirmations: u64, // The number of confirmations
    value: f64,         // The transaction value in BTC
    script_pub_key: ScriptPubKey,
    coinbase: bool, // Coinbase or not
}

pub struct GetTxOutCommand {
    tx_id: String,                 // (string, required) The transaction id
    n: u64,                        // (numeric, required) vout number
    include_mempool: Option<bool>, // (boolean, optional, default=true) Whether to include the mempool. Note that an unspent output that is spent in the mempool won't appear.
}
impl GetTxOutCommand {
    pub fn new(tx_id: String, n: u64) -> Self {
        GetTxOutCommand {
            tx_id,
            n,
            include_mempool: None,
        }
    }
    // TODO: Currently errors out if you don't enable mempool inclusion but search for a mempool
    // transaction
    pub fn include_mempool(&mut self, include_mempool: bool) -> &Self {
        self.include_mempool = Some(include_mempool);
        self
    }
}
impl CallableCommand for GetTxOutCommand {
    type Response = GetTxOutCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let tx_id_arg = &self.tx_id;
        let n_arg = &self.n;
        let include_mempool = &self.include_mempool;

        let tx_id_arg_raw_value = to_raw_value(&tx_id_arg).unwrap();
        let n_arg_raw_value = to_raw_value(&n_arg).unwrap();
        let mut params = vec![tx_id_arg_raw_value, n_arg_raw_value];
        if let Some(include_mempool_arg) = include_mempool {
            let include_mempool_arg_raw_value = to_raw_value(&include_mempool_arg).unwrap();
            params.push(include_mempool_arg_raw_value)
        }
        let r = request(client, GET_TX_OUT_COMMAND, params);
        let response: GetTxOutCommandResponse = r.result().unwrap();
        response
    }
}
