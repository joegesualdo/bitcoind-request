#![allow(dead_code)]
#![allow(unused_imports)]
use bitcoind_request::{
    client::Client,
    command::{
        get_best_block_hash::GetBestBlockHashCommand,
        get_block::{
            GetBlockCommand, GetBlockCommandResponse, GetBlockCommandTransactionResponse,
            GetBlockCommandVerbosity,
        },
        get_block_count::GetBlockCountCommand,
        get_block_hash::GetBlockHashCommand,
        get_block_header::GetBlockHeaderCommand,
        get_block_stats::{
            GetBlockStatsCommand, GetBlockStatsCommandResponse,
            GetBlockStatsCommandWithAllStatsResponse, StatsArgumentChoices, TargetBlockArgument,
        },
        get_blockchain_info::GetBlockchainInfoCommand,
        get_chain_tips::GetChainTipsCommand,
        get_chain_tx_stats::GetChainTxStatsCommand,
        get_difficulty::GetDifficultyCommand,
        get_mining_info::GetMiningInfoCommand,
        get_network_hash_ps::GetNetworkHashPsCommand,
        get_raw_transaction::{GetRawTransactionCommand, GetRawTransactionCommandResponse, Vin},
        get_tx_out::GetTxOutCommand,
        get_tx_out_set_info::GetTxOutSetInfoCommand,
        CallableCommand,
    },
};

use bitcoind_request::client;
use bitcoind_request::{Blockhash, BlockhashHexEncoded};

use chrono::{DateTime, TimeZone, Utc};
use jsonrpc::simple_http::{self, SimpleHttpTransport};
use std::time::{Duration, SystemTime};
use std::{env, time::SystemTimeError};

struct Seconds(pub i64);
fn format_duration(seconds: i64) -> String {
    let seconds_formatted = seconds % 60;
    let minutes_formatted = (seconds / 60) % 60;
    format!("{:#?}:{:#?}", minutes_formatted, seconds_formatted)
}

fn get_block_height(client: &Client) -> u64 {
    let block_count = GetBlockCountCommand::new().call(client);
    return block_count.0;
}

fn get_time_since_last_block(client: &Client) -> Seconds {
    let block_count = GetBlockCountCommand::new().call(client);
    let arg = TargetBlockArgument::Height(block_count.0);
    let block_stats_response = GetBlockStatsCommand::new(arg).call(client);
    let time_of_last_block = match block_stats_response {
        GetBlockStatsCommandResponse::AllStats(response) => response.time,
        GetBlockStatsCommandResponse::SelectiveStats(response) => response.time.unwrap(),
    };
    let current_datetime = chrono::offset::Utc::now();
    let current_timestamp = current_datetime.timestamp();
    let datetime_of_last_block = Utc.timestamp(time_of_last_block as i64, 0);
    let difference = current_datetime.signed_duration_since(datetime_of_last_block);
    Seconds(difference.num_seconds())

    //match SystemTime::now().duration_since(time_of_last_block) {
    //    Ok(seconds) => Ok(seconds.as_secs()),
    //    Err(err) => Err(err),
    //}
}

fn get_average_block_time(client: &Client) -> u64 {
    let blocks_to_calculate = 2016;
    let chain_tx_stats = GetChainTxStatsCommand::new()
        .set_n_blocks(2016)
        .call(client);
    let average_seconds_per_block = chain_tx_stats.window_interval / blocks_to_calculate;
    average_seconds_per_block
}

fn get_total_money_supply(client: &Client) -> u64 {
    // calls to gettxoutsetinfo are erroring out due to this: https://github.com/apoelstra/rust-jsonrpc/issues/67
    let tx_out_set_info = GetTxOutSetInfoCommand::new().call(client);
    tx_out_set_info.total_amount
}

// gets the chain size in bytes
fn get_chain_size(client: &Client) -> u64 {
    let blockchain_info = GetBlockchainInfoCommand::new().call(client);
    blockchain_info.size_on_disk
}

fn main() {
    let password = env::var("BITCOIND_PASSWORD").expect("BITCOIND_PASSWORD env variable not set");
    let username = env::var("BITCOIND_USERNAME").expect("BITCOIND_USERNAME env variable not set");
    let url = env::var("BITCOIND_URL").expect("BITCOIND_URL env variable not set");
    let client = Client::new(&url, &username, &password).expect("failed to create client");

    let block_height = get_block_height(&client);
    println!("BLOCK HEIGHT: {:#?}", block_height);

    let seconds_since_last_block = get_time_since_last_block(&client).0;
    println!(
        "TIME SINCE LAST BLOCK: {}",
        format_duration(seconds_since_last_block)
    );

    let average_seconds_per_block = get_average_block_time(&client);
    println!(
        "AVERAGE BLOCK TIME (2016): {}",
        format_duration(average_seconds_per_block as i64)
    );

    // Errors out
    // let total_money_supply = get_total_money_supply(&client);
    // println!("TOTAL MONEY SUPPLY: {:#?}", total_money_supply);

    let chain_size = get_chain_size(&client);
    let chain_size_in_gbs = chain_size as f64 / 1_000_000_000.0;
    println!("CHAIN SIZE: {:#?}GB", chain_size_in_gbs);

    let hash_rate = GetNetworkHashPsCommand::new()
        .set_n_blocks(
            bitcoind_request::command::get_network_hash_ps::BlocksToIncludeArg::NBlocks(2016),
        )
        .set_height(bitcoind_request::command::get_network_hash_ps::HeightArg::Height(block_height))
        .call(&client);
    println!("hash_rate:{:#?}", hash_rate)
}
