/*
getmempoolinfo

Returns details on the active state of the TX memory pool.

Result:
{                            (json object)
  "loaded" : true|false,     (boolean) True if the mempool is fully loaded
  "size" : n,                (numeric) Current tx count
  "bytes" : n,               (numeric) Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
  "usage" : n,               (numeric) Total memory usage for the mempool
  "total_fee" : n,           (numeric) Total fees for the mempool in BTC, ignoring modified fees through prioritizetransaction
  "maxmempool" : n,          (numeric) Maximum memory usage for the mempool
  "mempoolminfee" : n,       (numeric) Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
  "minrelaytxfee" : n,       (numeric) Current minimum relay fee for transactions
  "unbroadcastcount" : n     (numeric) Current number of transactions that haven't passed initial broadcast yet
}

Examples:
> bitcoin-cli getmempoolinfo
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getmempoolinfo", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::client::Client;
use crate::command::request::request;
use crate::command::CallableCommand;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::RawValue;

pub struct GetMempoolInfoCommand {}
impl GetMempoolInfoCommand {
    pub fn new() -> Self {
        GetMempoolInfoCommand {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMempoolInfoCommandResponse {
    loaded: bool,
    size: u64,
    bytes: u64,
    usage: u64,
    total_fee: f64,
    maxmempool: u64,
    mempoolminfee: f64,
    minrelaytxfee: f64,
    unbroadcastcount: u64,
}

impl CallableCommand for GetMempoolInfoCommand {
    type Response = GetMempoolInfoCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let command = "getmempoolinfo";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetMempoolInfoCommandResponse = r.result().unwrap();
        response
    }
}
