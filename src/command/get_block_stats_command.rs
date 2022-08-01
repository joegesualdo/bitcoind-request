/*
getblockstats hash_or_height ( stats )

Compute per block statistics for a given window. All amounts are in satoshis.
It won't work for some heights with pruning.

Arguments:
1. hash_or_height    (string or numeric, required) The block hash or height of the target block
2. stats             (json array, optional, default=all values) Values to plot (see result below)
     [
       "height",     (string) Selected statistic
       "time",       (string) Selected statistic
       ...
     ]

Result:
{                              (json object)
  "avgfee" : n,                (numeric) Average fee in the block
  "avgfeerate" : n,            (numeric) Average feerate (in satoshis per virtual byte)
  "avgtxsize" : n,             (numeric) Average transaction size
  "blockhash" : "hex",         (string) The block hash (to check for potential reorgs)
  "feerate_percentiles" : [    (json array) Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    n,                         (numeric) The 10th percentile feerate
    n,                         (numeric) The 25th percentile feerate
    n,                         (numeric) The 50th percentile feerate
    n,                         (numeric) The 75th percentile feerate
    n                          (numeric) The 90th percentile feerate
  ],
  "height" : n,                (numeric) The height of the block
  "ins" : n,                   (numeric) The number of inputs (excluding coinbase)
  "maxfee" : n,                (numeric) Maximum fee in the block
  "maxfeerate" : n,            (numeric) Maximum feerate (in satoshis per virtual byte)
  "maxtxsize" : n,             (numeric) Maximum transaction size
  "medianfee" : n,             (numeric) Truncated median fee in the block
  "mediantime" : n,            (numeric) The block median time past
  "mediantxsize" : n,          (numeric) Truncated median transaction size
  "minfee" : n,                (numeric) Minimum fee in the block
  "minfeerate" : n,            (numeric) Minimum feerate (in satoshis per virtual byte)
  "mintxsize" : n,             (numeric) Minimum transaction size
  "outs" : n,                  (numeric) The number of outputs
  "subsidy" : n,               (numeric) The block subsidy
  "swtotal_size" : n,          (numeric) Total size of all segwit transactions
  "swtotal_weight" : n,        (numeric) Total weight of all segwit transactions
  "swtxs" : n,                 (numeric) The number of segwit transactions
  "time" : n,                  (numeric) The block time
  "total_out" : n,             (numeric) Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee])
  "total_size" : n,            (numeric) Total size of all non-coinbase transactions
  "total_weight" : n,          (numeric) Total weight of all non-coinbase transactions
  "totalfee" : n,              (numeric) The fee total
  "txs" : n,                   (numeric) The number of transactions (including coinbase)
  "utxo_increase" : n,         (numeric) The increase/decrease in the number of unspent outputs
  "utxo_size_inc" : n          (numeric) The increase/decrease in size for the utxo index (not discounting op_return and similar)
}

Examples:
> bitcoin-cli getblockstats '"00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09"' '["minfeerate","avgfeerate"]'
> bitcoin-cli getblockstats 1000 '["minfeerate","avgfeerate"]'
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockstats", "params": ["00000000c937983704a73af28acdec37b049d214adbda81d7e2a3dd146f6ed09", ["minfeerate","avgfeerate"]]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockstats", "params": [1000, ["minfeerate","avgfeerate"]]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
 */
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;
use std::fmt;

use crate::{Blockhash, CallableCommand};

use super::request::request;

type BlockHeight = u64;
pub enum TargetBlockArgument {
    Hash(Blockhash),
    Height(BlockHeight),
}
// TODO: Fill in all of these:
//     https://bitcoincore.org/en/doc/0.21.0/rpc/blockchain/getblockstats/
#[derive(Serialize, Deserialize, Debug)]
pub enum StatsArgumentChoices {
    AvgFee,
    AvgTxSize,
    Blockhash,
    FeeRatePercentiles,
    Height,
    Ins,
    MaxFee,
    MaxFeeRate,
    MaxTxSize,
    MedianFee,
    MedianTime,
    MedianTxSize,
    MinFee,
    MinFeeRate,
    MinTxSize,
    Outs,
    Subsidy,
    SwTotalSize,
    SwTotalWeight,
    SwTxs,
    Time,
    TotalOut,
    TotalSize,
    TotalWeight,
    TotalFee,
    Txs,
    UtxoIncrease,
    UtxoSizeInc,
}
impl fmt::Display for StatsArgumentChoices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatsArgumentChoices::AvgFee => write!(f, "avgfee"),
            StatsArgumentChoices::AvgTxSize => write!(f, "avgtxsize"),
            StatsArgumentChoices::Blockhash => write!(f, "blockhash"),
            StatsArgumentChoices::FeeRatePercentiles => write!(f, "feerate_percentiles"),
            StatsArgumentChoices::Height => write!(f, "height"),
            StatsArgumentChoices::Ins => write!(f, "ins"),
            StatsArgumentChoices::MaxFee => write!(f, "maxfee"),
            StatsArgumentChoices::MaxFeeRate => write!(f, "maxfeerate"),
            StatsArgumentChoices::MaxTxSize => write!(f, "maxtxsize"),
            StatsArgumentChoices::MedianFee => write!(f, "medianfee"),
            StatsArgumentChoices::MedianTime => write!(f, "mediantime"),
            StatsArgumentChoices::MedianTxSize => write!(f, "mediantxsize"),
            StatsArgumentChoices::MinFee => write!(f, "minfee"),
            StatsArgumentChoices::MinFeeRate => write!(f, "minfeerate"),
            StatsArgumentChoices::MinTxSize => write!(f, "mintxsize"),
            StatsArgumentChoices::Outs => write!(f, "outs"),
            StatsArgumentChoices::Subsidy => write!(f, "subsidy"),
            StatsArgumentChoices::SwTotalSize => write!(f, "swtotal_size"),
            StatsArgumentChoices::SwTotalWeight => write!(f, "swtotal_weight"),
            StatsArgumentChoices::SwTxs => write!(f, "swtxs"),
            StatsArgumentChoices::Time => write!(f, "time"),
            StatsArgumentChoices::TotalOut => write!(f, "total_out"),
            StatsArgumentChoices::TotalSize => write!(f, "total_size"),
            StatsArgumentChoices::TotalWeight => write!(f, "total_weight"),
            StatsArgumentChoices::TotalFee => write!(f, "totalfee"),
            StatsArgumentChoices::Txs => write!(f, "txs"),
            StatsArgumentChoices::UtxoIncrease => write!(f, "utxo_increase"),
            StatsArgumentChoices::UtxoSizeInc => write!(f, "utxo_size_inc"),
        }
    }
}
pub struct GetBlockStatsCommand {
    target_block: TargetBlockArgument,
    stats: Vec<StatsArgumentChoices>,
}

impl GetBlockStatsCommand {
    pub fn new(target_block: TargetBlockArgument) -> Self {
        GetBlockStatsCommand {
            target_block,
            stats: vec![],
        }
    }
    pub fn add_selective_stats(
        &mut self,
        stats_argument_choices: Vec<StatsArgumentChoices>,
    ) -> &Self {
        self.stats = stats_argument_choices;
        self
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockStatsCommandWithSelectiveStatsResponse {
    avgfee: Option<u64>,                   // Average fee in the block
    avgfeerate: Option<u64>,               // Average feerate (in satoshis per virtual byte)
    avgtxsize: Option<u64>,                // Average transaction size
    blockhash: Option<String>,             // "hex" The block hash (to check for potential reorgs)
    feerate_percentiles: Option<[u64; 5]>, //  Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    //  index 0,                         (numeric) The 10th percentile feerate
    //  index 1                        (numeric) The 25th percentile feerate
    //  index 2                         (numeric) The 50th percentile feerate
    //  index 3,                         (numeric) The 75th percentile feerate
    //  index 4                         (numeric) The 90th percentile feerate
    height: Option<u64>,         // The height of the block
    ins: Option<u64>,            // The number of inputs (excluding coinbase)
    maxfee: Option<u64>,         // Maximum fee in the block
    maxfeerate: Option<u64>,     // Maximum feerate (in satoshis per virtual byte)
    maxtxsize: Option<u64>,      // Maximum transaction size
    medianfee: Option<u64>,      //Truncated median fee in the block
    mediantime: Option<u64>,     // The block median time past
    mediantxsize: Option<u64>,   // Truncated median transaction size
    minfee: Option<u64>,         // Minimum fee in the block
    minfeerate: Option<u64>,     // Minimum feerate (in satoshis per virtual byte)
    mintxsize: Option<u64>,      // Minimum transaction size
    outs: Option<u64>,           // The number of outputs
    subsidy: Option<u64>,        // The block subsidy
    swtotal_size: Option<u64>,   // Total size of all segwit transactions
    swtotal_weight: Option<u64>, // Total weight of all segwit transactions
    swtxs: Option<u64>,          // The number of segwit transactions
    time: Option<u64>,           // The block time
    total_out: Option<u64>, // Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee])
    total_size: Option<u64>, // Total size of all non-coinbase transactions
    total_weight: Option<u64>, // Total weight of all non-coinbase transactions
    totalfee: Option<u64>,  // The fee total
    txs: Option<u64>,       // The number of transactions (including coinbase)
    utxo_increase: Option<u64>, // The increase/decrease in the number of unspent outputs
    utxo_size_inc: Option<u64>, // The increase/decrease in size for the utxo index (not discounting op_return and similar)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockStatsCommandWithAllStatsResponse {
    avgfee: u64,                   // Average fee in the block
    avgfeerate: u64,               // Average feerate (in satoshis per virtual byte)
    avgtxsize: u64,                // Average transaction size
    blockhash: String,             // "hex" The block hash (to check for potential reorgs)
    feerate_percentiles: [u64; 5], //  Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    //  index 0,                         (numeric) The 10th percentile feerate
    //  index 1                        (numeric) The 25th percentile feerate
    //  index 2                         (numeric) The 50th percentile feerate
    //  index 3,                         (numeric) The 75th percentile feerate
    //  index 4                         (numeric) The 90th percentile feerate
    height: u64,         // The height of the block
    ins: u64,            // The number of inputs (excluding coinbase)
    maxfee: u64,         // Maximum fee in the block
    maxfeerate: u64,     // Maximum feerate (in satoshis per virtual byte)
    maxtxsize: u64,      // Maximum transaction size
    medianfee: u64,      //Truncated median fee in the block
    mediantime: u64,     // The block median time past
    mediantxsize: u64,   // Truncated median transaction size
    minfee: u64,         // Minimum fee in the block
    minfeerate: u64,     // Minimum feerate (in satoshis per virtual byte)
    mintxsize: u64,      // Minimum transaction size
    outs: u64,           // The number of outputs
    subsidy: u64,        // The block subsidy
    swtotal_size: u64,   // Total size of all segwit transactions
    swtotal_weight: u64, // Total weight of all segwit transactions
    swtxs: u64,          // The number of segwit transactions
    time: u64,           // The block time
    total_out: u64, // Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee])
    total_size: u64, // Total size of all non-coinbase transactions
    total_weight: u64, // Total weight of all non-coinbase transactions
    totalfee: u64,  // The fee total
    txs: u64,       // The number of transactions (including coinbase)
    utxo_increase: i64, // The increase/decrease in the number of unspent outputs
    utxo_size_inc: i64, // The increase/decrease in size for the utxo index (not discounting op_return and similar)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum GetBlockStatsCommandResponse {
    SelectiveStats(GetBlockStatsCommandWithSelectiveStatsResponse),
    AllStats(GetBlockStatsCommandWithAllStatsResponse),
}

impl CallableCommand for GetBlockStatsCommand {
    type Response = GetBlockStatsCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let target_block = &self.target_block;
        let hash_or_height_arg_raw_value = match target_block {
            TargetBlockArgument::Hash(hash) => to_raw_value(&hash).unwrap(),
            TargetBlockArgument::Height(height) => to_raw_value(&height).unwrap(),
        };

        // TODO: Add stats param!
        let stats_arg: Vec<String> = self.stats.iter().map(|stat| stat.to_string()).collect();
        let stats_arg_raw_value = to_raw_value(&stats_arg).unwrap();
        let command = "getblockstats";
        let params = vec![hash_or_height_arg_raw_value, stats_arg_raw_value];
        let r = request(client, command, params);
        let response: GetBlockStatsCommandResponse = if stats_arg.is_empty() {
            GetBlockStatsCommandResponse::AllStats(r.result().unwrap())
        } else {
            GetBlockStatsCommandResponse::SelectiveStats(r.result().unwrap())
        };
        response
    }
}
