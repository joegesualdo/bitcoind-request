use crate::client::Client;
/*
getchaintxstats ( nblocks "blockhash" )

Compute statistics about the total number and rate of transactions in the chain.

Arguments:
1. nblocks      (numeric, optional, default=one month) Size of the window in number of blocks
2. blockhash    (string, optional, default=chain tip) The hash of the block that ends the window.

Result:
{                                       (json object)
  "time" : xxx,                         (numeric) The timestamp for the final block in the window, expressed in UNIX epoch time
  "txcount" : n,                        (numeric) The total number of transactions in the chain up to that point
  "window_final_block_hash" : "hex",    (string) The hash of the final block in the window
  "window_final_block_height" : n,      (numeric) The height of the final block in the window.
  "window_block_count" : n,             (numeric) Size of the window in number of blocks
  "window_tx_count" : n,                (numeric) The number of transactions in the window. Only returned if "window_block_count" is > 0
  "window_interval" : n,                (numeric) The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0
  "txrate" : n                          (numeric) The average rate of transactions per second in the window. Only returned if "window_interval" is > 0
}

Examples:
> bitcoin-cli getchaintxstats
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getchaintxstats", "params": [2016]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
 */
use crate::command::request::request;
use crate::command::CallableCommand;
use crate::Blockhash;
use crate::BlockhashHexEncoded;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::{to_raw_value, RawValue};

pub struct GetChainTxStatsCommand {
    n_blocks: Option<u64>, // (numeric, optional, default=one month) Size of the window in number of blocks
    blockhash: Option<Blockhash>, //  (string, optional, default=chain tip) The hash of the block that ends the window.
}
impl GetChainTxStatsCommand {
    pub fn new() -> Self {
        GetChainTxStatsCommand {
            n_blocks: None, // defaults to one month
            blockhash: None,
        }
    }
    pub fn set_n_blocks(&mut self, n_blocks: u64) -> &Self {
        self.n_blocks = Some(n_blocks);
        self
    }
    pub fn set_blockhash(&mut self, blockhash: Blockhash) -> &Self {
        self.blockhash = Some(blockhash);
        self
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetChainTxStatsCommandResponse {
    pub time: u64, // The timestamp for the final block in the window, expressed in UNIX epoch time
    pub txcount: u64, // The total number of transactions in the chain up to that point
    pub window_final_block_hash: String, // "hex" The hash of the final block in the window
    pub window_final_block_height: u64, // The height of the final block in the window.
    pub window_block_count: u64, // Size of the window in number of blocks
    pub window_tx_count: u64, // The number of transactions in the window. Only returned if "window_block_count" is > 0
    pub window_interval: u64, // The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0
    pub txrate: f64, // The average rate of transactions per second in the window. Only returned if "window_interval" is > 0
}

impl CallableCommand for GetChainTxStatsCommand {
    type Response = GetChainTxStatsCommandResponse;
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error> {
        let command = "getchaintxstats";
        let mut params: Vec<Box<RawValue>> = vec![];
        if let Some(n_blocks) = &self.n_blocks {
            let n_blocks_arg_raw_value = to_raw_value(&n_blocks).unwrap();
            params.push(n_blocks_arg_raw_value)
        }
        if let Some(blockhash) = &self.blockhash {
            let blockhash_str = &blockhash.0;
            let blockhash_arg_raw_value = to_raw_value(&blockhash_str).unwrap();
            params.push(blockhash_arg_raw_value)
        }
        let r = request(client, command, params);
        let response: GetChainTxStatsCommandResponse = r.result()?;
        Ok(response)
    }
}
