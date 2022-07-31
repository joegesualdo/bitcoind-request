use bitcoin_request::{
    BlockhashHexEncoded, CallableCommand, GetBestBlockHashCommand, GetBlockCommand,
    GetBlockCommandResponse, GetBlockCommandTransactionResponse, GetBlockCommandVerbosity,
    GetBlockCountCommand, GetBlockHashCommand, GetBlockHeaderCommand, GetBlockchainInfoCommand,
    GetRawTransactionCommand, GetRawTransactionCommandResponse, Vin,
};
use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;
use std::env;

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
) -> (f64, f64) {
    match get_block_command_response {
        GetBlockCommandResponse::Block(block) => {
            let mut total_vin_value = 0.0;
            let mut total_vout_value = 0.0;
            let mut total_coinbase_vout_value = 0.0;
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
                                                GetRawTransactionCommandResponse::SerializedHexEncodedData(_s) => {}
                                                GetRawTransactionCommandResponse::Transaction(t) => {
                                                    let vin_vout_index = v.vout as usize;
                                                    let vout = &t.vout[vin_vout_index];
                                                    total_vin_value= total_vin_value+ vout.value;
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
                                    }
                                }
                            }
                        }
                    }
                    GetBlockCommandTransactionResponse::Id(id) => {}
                }
            }

            println!("Total coinbase: {:#?}", total_coinbase_vout_value);
            let difference = total_vin_value - total_vout_value;
            (difference, total_coinbase_vout_value)
        }
        //println!("hash: {:#?}", block.hash);
        //println!("size: {:#?}mb", block.size as f64 / 1000000.0);
        //println!("size: {:#?}mwu", block.weight as f64 / 1000000.0);
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
    // let newest_block = GetBlockCommand::new(newest_block_hash)
    //    .verbosity(GetBlockCommandVerbosity::BlockObjectWithTransactionInformation)
    //    .call(&client);
    // let (total_fees, total_subsidy) = get_total_fees_for_block(&client, newest_block);
    // println!("Total fees are: {:#?} BTC", total_fees);
    // println!("Total subsidy: {:#?} BTC", total_subsidy);
    // println!("block reward: {:#?} BTC", total_subsidy - total_fees);

    let blockchaininfo_response = GetBlockchainInfoCommand::new().call(&client);
    println!("{:#?}", blockchaininfo_response);
    let block_header_response = GetBlockHeaderCommand::new(newest_block_hash).call(&client);
    println!("{:#?}", block_header_response);
}
