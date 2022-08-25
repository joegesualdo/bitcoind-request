/*
getmempoolentry "txid"

Returns mempool data for given transaction

Arguments:
1. txid    (string, required) The transaction id (must be in mempool)

Result:
{                                       (json object)
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
}

Examples:
> bitcoin-cli getmempoolentry "mytxid"
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getmempoolentry", "params": ["mytxid"]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;

use crate::client::Client;
use crate::{command::CallableCommand, Blockhash};

use crate::command::request::request;

type TxId = String;
pub struct GetMempoolEntryCommand {
    txid: TxId,
}

impl GetMempoolEntryCommand {
    pub fn new(txid: TxId) -> Self {
        GetMempoolEntryCommand { txid }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Fees {
    pub base: f64,
    pub modified: f64,
    pub ancestor: f64,
    pub descendant: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMempoolEntryCommandResponse {
    pub vsize: u64,
    pub weight: u64,
    pub fee: f64,
    pub modifiedfee: f64,
    // TODO: represent using unix time
    pub time: u64,
    pub height: u64,
    pub descendantcount: u64,
    pub descendantsize: u64,
    pub descendantfees: u64,
    pub ancestorcount: u64,
    pub ancestorsize: u64,
    pub ancestorfees: u64,
    // TODO: represent using hex
    pub wtxid: String,
    pub fees: Fees,
    pub depends: Vec<String>,
    pub spentby: Vec<String>,
    #[serde(alias = "bip125-replaceable")]
    pub bip125_replaceable: bool,
    pub unbroadcast: bool,
}

impl CallableCommand for GetMempoolEntryCommand {
    type Response = GetMempoolEntryCommandResponse;
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error> {
        let txid_arg = &self.txid;
        // TODO: Add blockhas param!
        //let blockhash_arg = &self.blockhash.0;
        let txid_arg_raw_value = to_raw_value(&txid_arg).unwrap();
        let command = "getmempoolentry";
        let params = vec![txid_arg_raw_value];
        let r = request(client, command, params);
        let response: GetMempoolEntryCommandResponse = r.result()?;
        Ok(response)
    }
}
