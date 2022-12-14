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
        get_connection_count::GetConnectionCountCommand,
        get_difficulty::GetDifficultyCommand,
        get_mempool_entry::GetMempoolEntryCommand,
        get_mempool_info::GetMempoolInfoCommand,
        get_mining_info::GetMiningInfoCommand,
        get_network_hash_ps::GetNetworkHashPsCommand,
        get_network_info::GetNetworkInfoCommand,
        get_node_addresses::{CountArg, GetNodeAddressesCommand, NetworkArg},
        get_peer_info::GetPeerInfoCommand,
        get_raw_mempool::GetRawMempoolCommand,
        get_raw_transaction::{GetRawTransactionCommand, GetRawTransactionCommandResponse, Vin},
        get_tx_out::GetTxOutCommand,
        get_tx_out_set_info::GetTxOutSetInfoCommand,
        CallableCommand,
    },
};

use bitcoind_request::client;
use bitcoind_request::{Blockhash, BlockhashHexEncoded};

use chrono::{DateTime, Duration, TimeZone, Utc};
use jsonrpc::simple_http::{self, SimpleHttpTransport};
use std::time::SystemTime;
use std::{env, time::SystemTimeError};

struct Seconds(pub i64);
fn format_duration(seconds: i64) -> String {
    let seconds_formatted = seconds % 60;
    let minutes_formatted = (seconds / 60) % 60;
    format!("{:#?}:{:#?}", minutes_formatted, seconds_formatted)
}

fn get_block_height(client: &Client) -> u64 {
    let block_count = GetBlockCountCommand::new().call(client);
    return block_count.unwrap().0;
}

fn get_time_since_last_block(client: &Client) -> Seconds {
    let block_count = GetBlockCountCommand::new().call(client);
    let arg = TargetBlockArgument::Height(block_count.unwrap().0);
    let maybe_block_stats_response = GetBlockStatsCommand::new(arg).call(client);
    let time_of_last_block = match maybe_block_stats_response {
        Ok(block_stats_response) => match block_stats_response {
            GetBlockStatsCommandResponse::AllStats(response) => response.time,
            GetBlockStatsCommandResponse::SelectiveStats(response) => response.time.unwrap(),
        },
        Err(_) => panic!("panic"),
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
    let maybe_chain_tx_stats = GetChainTxStatsCommand::new()
        .set_n_blocks(2016)
        .call(client);
    let average_seconds_per_block =
        maybe_chain_tx_stats.unwrap().window_interval / blocks_to_calculate;
    average_seconds_per_block
}

fn get_total_money_supply(client: &Client) -> f64 {
    // calls to gettxoutsetinfo are erroring out due to this: https://github.com/apoelstra/rust-jsonrpc/issues/67
    let tx_out_set_info = GetTxOutSetInfoCommand::new().call(client);
    tx_out_set_info.unwrap().total_amount
}

// gets the chain size in bytes
fn get_chain_size(client: &Client) -> u64 {
    let blockchain_info = GetBlockchainInfoCommand::new().call(client);
    blockchain_info.unwrap().size_on_disk
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

    // let chain_size = get_chain_size(&client);
    // let chain_size_in_gbs = chain_size as f64 / 1_000_000_000.0;
    // println!("CHAIN SIZE: {:#?}GB", chain_size_in_gbs);

    // let hash_rate = GetNetworkHashPsCommand::new()
    //     .set_n_blocks(
    //         bitcoind_request::command::get_network_hash_ps::BlocksToIncludeArg::NBlocks(2016),
    //     )
    //     .set_height(bitcoind_request::command::get_network_hash_ps::HeightArg::Height(block_height))
    //     .call(&client);
    // println!("hash_rate:{:#?}", hash_rate);

    // let connection_count = GetConnectionCountCommand::new().call(&client);
    // println!("connection_count:{:#?}", connection_count);

    // let node_addresses = GetNodeAddressesCommand::new()
    //     .set_count(CountArg::AllAddresses)
    //     .set_network(NetworkArg::All)
    //     .call(&client);
    // println!("node addresses:{:#?}", node_addresses.unwrap().0);
    // let mut reachable_nodes = 0;
    // node_addresses.unwrap().0.iter().for_each(|node| {
    //     let current_datetime = chrono::offset::Utc::now();
    //     let datetime_of_node = Utc.timestamp(node.time as i64, 0);
    //     let difference: Duration = current_datetime.signed_duration_since(datetime_of_node);
    //     let seconds = difference.num_seconds();
    //     let seconds_in_a_day = 60 * 60 * 24;
    //     if seconds < seconds_in_a_day {
    //         reachable_nodes = reachable_nodes + 1;
    //     }
    // });
    // println!("reachable nodes count: {}", reachable_nodes);

    // let peer_info = GetPeerInfoCommand::new().call(&client);
    // println!("peerinfo:{:#?}", peer_info.unwrap().0.last());

    // let network_info = GetNetworkInfoCommand::new().call(&client);
    // println!("network info:{:#?}", network_info);

    // let mempool_info = GetMempoolInfoCommand::new().call(&client);
    // println!("mempool info:{:#?}", mempool_info);

    // let raw_mempool = GetRawMempoolCommand::new()
    //     .set_verbose(true)
    //     .set_mempool_sequence(false)
    //     .call(&client);
    // println!("raw_mempool:{:#?}", raw_mempool);

    // what happens if the txid is no longer in the mempool
    //let mempool_entry = GetMempoolEntryCommand::new(
    //    "cbcedc2a784311f24c7cce95faae32fab093b2e98417d79db1eb9620115206e7".to_string(),
    //)
    //.call(&client);
    //let mempool_entry = GetMempoolEntryCommand::new(
    //    "cbcedc2a784311f24c7cce95faae32fab093b2e98417d79db1eb9620115206e7".to_string(),
    //)
    //.call(&client);
    //println!("mempool entry:{:#?}", mempool_entry);
    //
    let maybe_get_block_command_response = GetBlockCommand::new(Blockhash(
        "0000000000000000000137aa8bf31a6b8ad42ce1c08c33acfc033f97f0ef2bc7".to_string(),
    ))
    .verbosity(GetBlockCommandVerbosity::BlockObjectWithTransactionInformation)
    .call(&client);

    match maybe_get_block_command_response {
        Ok(get_block_command_response) => match get_block_command_response {
            GetBlockCommandResponse::Block(block) => {
                println!("height: {}", block.height);
                println!("hash: {}", block.hash);
                println!("time: {}", block.time);
                println!("size: {}", block.size);
                println!("weight: {}", block.weight);
            }
            GetBlockCommandResponse::BlockHash(hash) => panic!("not supported"),
        },
        Err(err) => {}
    }

    // let block = GetBlockCommand::new(Blockhash(
    //     "000000000000000000010887fdbbc731013853dde72c31110dc7130606df9474".to_string(),
    // ))
    // .verbosity(GetBlockCommandVerbosity::BlockObjectWithTransactionInformation)
    // .call(&client);
    // println!("mempool entry:{:#?}", block);

    // let transaction = GetRawTransactionCommand::new(
    //     "ef851362b06934b0082d4e2ea8a544c1982002deacef65198e18dc85a73aa49e".to_string(),
    // )
    // .verbose(true)
    // .call(&client);
    // println!("mempool entry:{:#?}", transaction);
    let tx_out_set_info = GetTxOutSetInfoCommand::new().call(&client);
    let x = tx_out_set_info.unwrap().txouts;
    println!("{}", x);
}
