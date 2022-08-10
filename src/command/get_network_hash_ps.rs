/*
getnetworkhashps ( nblocks height )

Returns the estimated network hashes per second based on the last n blocks.
Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
Pass in [height] to estimate the network speed at the time when a certain block was found.

Arguments:
1. nblocks    (numeric, optional, default=120) The number of blocks, or -1 for blocks since last difficulty change.
2. height     (numeric, optional, default=-1) To estimate at the time of the given height.

Result:
n    (numeric) Hashes per second estimated

Examples:
> bitcoin-cli getnetworkhashps
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getnetworkhashps", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::{request::request, CallableCommand};
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;

const GET_NETWORK_HASH_PS_COMMAND: &str = "getnetworkhashps";
const DEFAULT_N_BLOCKS: u64 = 120;
const N_BLOCKS_ARGUMENT_FOR_BLOCKS_SINCE_LAST_DIFFICULT_CHANGE: i64 = -1;
const HEIGHT_ARGUMENT_FOR_CALCULATING_BASED_ON_CURRENT_HEIGHT: i64 = -1;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNetworkHashPsCommandResponse(pub f64);

pub enum BlocksToIncludeArg {
    NBlocks(u64),
    BlocksSinceLastDifficultyChange,
}

pub enum HeightArg {
    Height(u64),
    CurrentHeight,
}
pub struct GetNetworkHashPsCommand {
    n_blocks: BlocksToIncludeArg, // (numeric, optional, default=120) The number of blocks, or -1 for blocks since last difficulty change.
    height: HeightArg, //(numeric, optional, default=-1) To estimate at the time of the given height.
}
impl GetNetworkHashPsCommand {
    pub fn new() -> Self {
        GetNetworkHashPsCommand {
            n_blocks: BlocksToIncludeArg::NBlocks(DEFAULT_N_BLOCKS), // default of 120 is from the docs: https://bitcoincore.org/en/doc/0.21.0/rpc/mining/getnetworkhashps/
            height: HeightArg::CurrentHeight, // default of -1 (which means current height), is
                                              // defined in the docs: https://bitcoincore.org/en/doc/0.21.0/rpc/mining/getnetworkhashps/
        }
    }
    pub fn set_n_blocks(mut self, n_blocks: BlocksToIncludeArg) -> Self {
        self.n_blocks = n_blocks;
        self
    }
    pub fn set_height(mut self, height: HeightArg) -> Self {
        self.height = height;
        self
    }
}
impl CallableCommand for GetNetworkHashPsCommand {
    type Response = GetNetworkHashPsCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let mut params: Vec<i64> = vec![];
        let n_blocks_arg: i64 = match self.n_blocks {
            BlocksToIncludeArg::NBlocks(n_blocks) => n_blocks as i64,
            BlocksToIncludeArg::BlocksSinceLastDifficultyChange => {
                N_BLOCKS_ARGUMENT_FOR_BLOCKS_SINCE_LAST_DIFFICULT_CHANGE
            }
        };
        let height_arg = match self.height {
            HeightArg::Height(height) => height as i64,
            HeightArg::CurrentHeight => HEIGHT_ARGUMENT_FOR_CALCULATING_BASED_ON_CURRENT_HEIGHT,
        };

        let n_blocks_arg_raw_value = to_raw_value(&n_blocks_arg).unwrap();
        let height_arg_raw_value = to_raw_value(&height_arg).unwrap();
        let mut params = vec![n_blocks_arg_raw_value, height_arg_raw_value];
        let r = request(client, GET_NETWORK_HASH_PS_COMMAND, params);
        let response: GetNetworkHashPsCommandResponse = r.result().unwrap();
        response
    }
}
