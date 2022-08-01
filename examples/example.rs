use bitcoin_request::{
    Blockhash, BlockhashHexEncoded, CallableCommand, GetBestBlockHashCommand, GetBlockCommand,
    GetBlockCommandResponse, GetBlockCommandTransactionResponse, GetBlockCommandVerbosity,
    GetBlockCountCommand, GetBlockHashCommand, GetBlockHeaderCommand, GetBlockchainInfoCommand,
    GetRawTransactionCommand, GetRawTransactionCommandResponse, Vin,
};
use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;
use std::env;

fn mean(numbers: &Vec<i32>) -> f32 {
    let sum: i32 = numbers.iter().sum();

    sum as f32 / numbers.len() as f32
}
fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }
}

fn client(url: &str, user: &str, pass: &str) -> Result<Client, simple_http::Error> {
    let t = SimpleHttpTransport::builder()
        .url(url)?
        .auth(user, Some(pass))
        .build();
    Ok(Client::with_transport(t))
}

// Returns total fees and total subsidy
fn get_total_fees_for_block(
    client: &Client,
    get_block_command_response: GetBlockCommandResponse,
) -> (f64, f64, Vec<i32>) {
    match get_block_command_response {
        GetBlockCommandResponse::Block(block) => {
            let mut total_vin_value = 0.0;
            let mut total_vout_value = 0.0;
            let mut total_coinbase_vout_value = 0.0;
            let transaction_count = block.tx.len();
            let mut sats_per_bytes: Vec<i32> = vec![];
            for tx in block.tx.into_iter().rev() {
                match tx {
                    GetBlockCommandTransactionResponse::Raw(transaction) => {
                        let txid = transaction.txid;
                        let full_transaction = GetRawTransactionCommand::new(txid)
                            .verbose(true)
                            .call(&client);
                        match full_transaction {
                            GetRawTransactionCommandResponse::SerializedHexEncodedData(_s) => {}
                            GetRawTransactionCommandResponse::Transaction(t) => {
                                let top_level_txid = &t.txid;
                                let mut transaction_vin_value = 0.0;
                                let mut transaction_vout_value = 0.0;
                                for vin in &t.vin {
                                    match vin {
                                        Vin::Coinbase(cbv) => {
                                            // DO NOTHING
                                        }
                                        Vin::NonCoinbase(v) => {
                                            let vin_transaction =
                                                GetRawTransactionCommand::new(v.txid.clone())
                                                    .verbose(true)
                                                    .call(&client);
                                            match vin_transaction {
                                                GetRawTransactionCommandResponse::SerializedHexEncodedData(_s) => {

                                                }
                                                GetRawTransactionCommandResponse::Transaction(t) => {
                                                    let vin_vout_index = v.vout as usize;
                                                    let vout = &t.vout[vin_vout_index];
                                                    total_vin_value= total_vin_value+ vout.value;
                                                    transaction_vin_value= transaction_vin_value + vout.value;
                                                }
                                            }
                                        }
                                    }
                                }
                                if t.is_coinbase_transaction() {
                                    for vout in &t.vout {
                                        total_coinbase_vout_value =
                                            vout.value + total_coinbase_vout_value
                                    }
                                } else {
                                    for vout in &t.vout {
                                        total_vout_value = total_vout_value + vout.value;
                                        transaction_vout_value =
                                            transaction_vout_value + vout.value;
                                    }
                                }
                                let fee_for_transaction =
                                    transaction_vin_value - transaction_vout_value;
                                //println!("fee: {:?}", fee_for_transaction);
                                let virtual_size = t.vsize;
                                //println!("virtual_size: {:?}", virtual_size);
                                let sats_per_byte = (fee_for_transaction * 100_000_000.0) as f64
                                    / virtual_size as f64;
                                sats_per_bytes.push(sats_per_byte as i32);
                            }
                        }
                    }
                    GetBlockCommandTransactionResponse::Id(id) => {}
                }
            }

            //println!("Total coinbase: {:#?}", total_coinbase_vout_value);
            let difference: f64 = total_vin_value - total_vout_value;
            println!("transaction_count: {}", transaction_count);
            let median_sats_per_byte = median(&mut sats_per_bytes);
            println!("media transaction fee_per_byte: {:?}", median_sats_per_byte);
            (difference, total_coinbase_vout_value, sats_per_bytes)
        }
        GetBlockCommandResponse::BlockHash(hash) => {
            todo!();
        }
    }
}

fn main() {
    let password = env::var("BITCOIND_PASSWORD").expect("BITCOIND_PASSWORD env variable not set");
    let username = env::var("BITCOIND_USERNAME").expect("BITCOIND_USERNAME env variable not set");
    let client = client("127.0.0.1:8332", &username, &password).expect("failed to create client");
    let blockhash_hex_encoded = BlockhashHexEncoded(
        "00000000000000000007d1712328c3b95adc170e3e04b2499c04a4ee2905f72e".to_string(),
    );

    let best_block_hash_response =
        GetBestBlockHashCommand::new(blockhash_hex_encoded).call(&client);
    println!("{:?}", best_block_hash_response);

    let best_block_hash = best_block_hash_response.0;

    let response = GetBlockCommand::new(best_block_hash)
        .verbosity(GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation)
        .call(&client);
    println!("{:#?}", response);

    let block_count = GetBlockCountCommand::new().call(&client);
    println!("{:#?}", block_count);

    let newest_block_hash_response = GetBlockHashCommand::new(block_count.0).call(&client);
    println!("{:#?}", newest_block_hash_response);

    let newest_block_hash = newest_block_hash_response.0;
    let newest_block = GetBlockCommand::new(newest_block_hash)
        .verbosity(GetBlockCommandVerbosity::BlockObjectWithTransactionInformation)
        .call(&client);
    let b = GetBlockCommand::new(Blockhash(
        "0000000000000000000821ac160f88b20f6d8741f88e92ff34cd8362bce7bf58".to_string(),
    ))
    .verbosity(GetBlockCommandVerbosity::BlockObjectWithTransactionInformation)
    .call(&client);
    let mut inputs = 0;
    let mut outputs = 0;
    let mut transaction_count = 0;
    let mut virtual_bytes = 0.0;
    match &b {
        GetBlockCommandResponse::Block(block) => {
            for transaction in &block.tx {
                let mut is_segwit = false;
                let mut is_coinbase = false;
                //println!("START -----------------------");
                transaction_count = transaction_count + 1;
                match transaction {
                    GetBlockCommandTransactionResponse::Raw(transaction) => {
                        for v in &transaction.vin {
                            if !is_segwit {
                                is_segwit = v.txinwitness.is_some();
                            } else {
                            }
                            if !is_coinbase {
                                is_coinbase = v.coinbase.is_some();
                            }
                        }
                        let input_count = transaction.vin.len();
                        let output_count = transaction.vout.len();
                        if !is_coinbase {
                            if is_segwit {
                                virtual_bytes =
                                    virtual_bytes + (transaction.weight as f64 / 4.0) as f64;
                            } else {
                                virtual_bytes = virtual_bytes + (transaction.weight) as f64;
                            }
                        }
                        //if !is_coinbase {
                        //virtual_bytes = virtual_bytes + transaction.vsize as f64;
                        //println!("Coinbase: {:?}", transaction);
                        //println!("Coinbase: {:?}", transaction.vin)
                        //}
                        //if is_segwit {
                        //    //virtual_bytes = 140.0
                        //    virtual_bytes = virtual_bytes
                        //        + ((input_count as f64 * 68.5)
                        //            + (output_count as f64 * 31.0)
                        //            + 10.0);
                        //    //virtual_bytes = virtual_bytes + (transaction.weight as f64 / 4.0);
                        //    //virtual_bytes = virtual_bytes + (transaction.size as f64 / 4.0);
                        //} else {
                        //    //virtual_bytes = virtual_bytes + (transaction.size as f64);
                        //    virtual_bytes = virtual_bytes
                        //        + (input_count as f64 * 146.0 + output_count as f64 * 33.0 + 10.0);
                        //}
                    }
                    GetBlockCommandTransactionResponse::Id(_s) => {}
                }
                //println!("End-----------------------");
            }
        }
        GetBlockCommandResponse::BlockHash(_hash) => (),
    };
    let (total_fees, total_subsidy, sats_per_bytes) = get_total_fees_for_block(&client, b);
    println!("sats per bytes: {:#?}", sats_per_bytes);
    println!("Total fees are: {:#?} BTC", total_fees);

    //println!("Total subsidy: {:#?} BTC", total_subsidy);
    //println!("block reward: {:#?} BTC", total_subsidy - total_fees);

    // let blockchaininfo_response = GetBlockchainInfoCommand::new().call(&client);
    // println!("{:#?}", blockchaininfo_response);
    // let block_header_response = GetBlockHeaderCommand::new(newest_block_hash).call(&client);
    // println!("{:#?}", block_header_response);
}
