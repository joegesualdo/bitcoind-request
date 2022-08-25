use std::collections::HashMap;

/*
getrawmempool ( verbose mempool_sequence )

Returns all transaction ids in memory pool as a json array of string transaction ids.

Hint: use getmempoolentry to fetch a specific transaction from the mempool.

Arguments:
1. verbose             (boolean, optional, default=false) True for a json object, false for array of transaction ids
2. mempool_sequence    (boolean, optional, default=false) If verbose=false, returns a json object with transaction list and mempool sequence number attached.

Result (for verbose = false):
[           (json array)
  "hex",    (string) The transaction id
  ...
]

Result (for verbose = true):
{                                         (json object)
  "transactionid" : {                     (json object)
    "vsize" : n,                          (numeric) virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    "weight" : n,                         (numeric) transaction weight as defined in BIP 141.
    "fee" : n,                            (numeric) transaction fee in BTC (DEPRECATED)
    "modifiedfee" : n,                    (numeric) transaction fee with fee deltas used for mining priority (DEPRECATED)
    "time" : xxx,                         (numeric) local time transaction entered pool in seconds since 1 Jan 1970 GMT
    "height" : n,                         (numeric) block height when transaction entered pool
    "descendantcount" : n,                (numeric) number of in-mempool descendant transactions (including this one)
    "descendantsize" : n,                 (numeric) virtual transaction size of in-mempool descendants (including this one)
    "descendantfees" : n,                 (numeric) modified fees (see above) of in-mempool descendants (including this one) (DEPRECATED)
    "ancestorcount" : n,                  (numeric) number of in-mempool ancestor transactions (including this one)
    "ancestorsize" : n,                   (numeric) virtual transaction size of in-mempool ancestors (including this one)
    "ancestorfees" : n,                   (numeric) modified fees (see above) of in-mempool ancestors (including this one) (DEPRECATED)
    "wtxid" : "hex",                      (string) hash of serialized transaction, including witness data
    "fees" : {                            (json object)
      "base" : n,                         (numeric) transaction fee in BTC
      "modified" : n,                     (numeric) transaction fee with fee deltas used for mining priority in BTC
      "ancestor" : n,                     (numeric) modified fees (see above) of in-mempool ancestors (including this one) in BTC
      "descendant" : n                    (numeric) modified fees (see above) of in-mempool descendants (including this one) in BTC
    },
    "depends" : [                         (json array) unconfirmed transactions used as inputs for this transaction
      "hex",                              (string) parent transaction id
      ...
    ],
    "spentby" : [                         (json array) unconfirmed transactions spending outputs from this transaction
      "hex",                              (string) child transaction id
      ...
    ],
    "bip125-replaceable" : true|false,    (boolean) Whether this transaction could be replaced due to BIP125 (replace-by-fee)
    "unbroadcast" : true|false            (boolean) Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
  },
  ...
}

Result (for verbose = false and mempool_sequence = true):
{                            (json object)
  "txids" : [                (json array)
    "hex",                   (string) The transaction id
    ...
  ],
  "mempool_sequence" : n     (numeric) The mempool sequence value.
}

Examples:
> bitcoin-cli getrawmempool true
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getrawmempool", "params": [true]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
 */
use crate::client::Client;
use crate::command::{request::request, CallableCommand};
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;

pub type TransactionId = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GetRawMempoolCommandResponse {
    TransacationIds(Vec<TransactionId>),
    TransactionIdsWithSequence(TransactionIdsWithSequence),
    Transactions(HashMap<TransactionId, Transaction>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionIdsWithSequence {
    pub txids: Vec<TransactionId>,
    pub mempool_sequence: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fees {
    pub base: f64,
    pub modified: f64,
    pub ancestor: f64,
    pub descendant: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub vsize: u64,
    pub weight: u64,
    pub fee: f64,
    pub modifiedfee: f64,
    // TODO: Represent using a unix time
    pub time: u64,
    pub height: u64,
    pub descendantcount: u64,
    pub descendantsize: u64,
    pub descendantfees: f64,
    pub ancestorcount: u64,
    pub ancestorsize: u64,
    pub ancestorfees: u64,
    pub wtxid: String,
    pub fees: Fees,
    pub depends: Vec<String>,
    pub spentby: Vec<String>,
    #[serde(alias = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    pub unbroadcast: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions(pub HashMap<TransactionId, Transaction>);

pub struct GetRawMempoolCommand {
    verbose: bool,
    mempool_sequence: bool,
}
impl GetRawMempoolCommand {
    pub fn new() -> Self {
        GetRawMempoolCommand {
            verbose: false,
            mempool_sequence: false,
        }
    }
    pub fn set_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    pub fn set_mempool_sequence(mut self, mempool_sequence: bool) -> Self {
        self.mempool_sequence = mempool_sequence;
        self
    }
}
impl CallableCommand for GetRawMempoolCommand {
    type Response = GetRawMempoolCommandResponse;
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error> {
        if self.verbose && self.mempool_sequence {
            panic!("RPC command 'getrawmempool' has invalid arguments of verbose=true & mempool_sequence=true. Verbose results cannot contain mempool sequence values.");
        }
        let verbose_arg = self.verbose;
        let mempool_sequence_arg = &self.mempool_sequence;
        let verbose_arg_raw_value = to_raw_value(&verbose_arg).unwrap();
        let mempool_sequence_arg_raw_value = to_raw_value(&mempool_sequence_arg).unwrap();
        let command = "getrawmempool";
        let params = vec![verbose_arg_raw_value, mempool_sequence_arg_raw_value];
        let r = request(client, command, params);
        let response: GetRawMempoolCommandResponse = r.result()?;
        Ok(response)
    }
}
