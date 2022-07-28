use jsonrpc::simple_http::{self, SimpleHttpTransport};
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;
use std::env;

type Hash = String;
#[derive(Serialize, Deserialize, Debug)]
struct GetMiningInfoResponse {
    blocks: isize,
    difficulty: f64,
    networkhashps: f64,
    pooledtx: isize,
    chain: String,
    warnings: String,
}

fn get_mining_info(client: &Client) -> GetMiningInfoResponse {
    let getmininginfo_request = client.build_request("getmininginfo", &[]);
    let response = client
        .send_request(getmininginfo_request)
        .expect("send_request failed");
    let getmininginfo_response: GetMiningInfoResponse = response.result().unwrap();
    getmininginfo_response
}

type GetBlockCountResponse = i64;

fn get_block_count(client: &Client) -> GetBlockCountResponse {
    let request = client.build_request("getmininginfo", &[]);
    let getblockcount_request = client.build_request("getblockcount", &[]);
    let response = client
        .send_request(getblockcount_request)
        .expect("send_request failed");
    let getblockcount_response: GetBlockCountResponse = response.result().unwrap();
    getblockcount_response
}

type GetBlockHashResponse = Hash;
fn get_block_hash(client: &Client, blockheight: &i64) -> GetBlockHashResponse {
    let raw = &[to_raw_value(&blockheight).unwrap()];
    let getblockhash_request = client.build_request("getblockhash", raw);
    let response = client
        .send_request(getblockhash_request)
        .expect("send_request failed");
    let getblockhash_response: GetBlockHashResponse = response.result().unwrap();
    getblockhash_response
}

fn client(url: &str, user: &str, pass: &str) -> Result<Client, simple_http::Error> {
    let t = SimpleHttpTransport::builder()
        .url(url)?
        .auth(user, Some(pass))
        .build();

    Ok(Client::with_transport(t))
}

#[derive(Serialize, Deserialize, Debug)]
struct GetBlockResponse {
    hash: String,
    confirmations: u64,
    height: u64,
    version: u64,
    versionHex: String,
    merkleroot: String,
    time: u64,
    mediantime: u64,
    nonce: u64,
    bits: String,
    difficulty: f64,
    chainwork: String,
    nTx: u64,
    previousblockhash: Option<String>,
    // TODO: Why isn't this always there?
    nextblockhash: Option<String>,
    strippedsize: u64,
    size: u64,
    weight: u64,
    tx: Vec<String>,
}
fn get_block(client: &Client, hash: String) -> GetBlockResponse {
    let raw = &[to_raw_value(&hash).unwrap()];
    let getblock_request = client.build_request("getblock", raw);
    let response = client
        .send_request(getblock_request)
        .expect("send_request failed");
    let getblock_response: GetBlockResponse = response.result().unwrap();
    getblock_response
}

type GetRawTransactionRepsonse = Hash;
fn get_raw_transaction(client: &Client, tx_hash: &Hash) -> GetRawTransactionRepsonse {
    let raw = &[to_raw_value(&tx_hash).unwrap()];
    let getrawtransaction_request = client.build_request("getrawtransaction", raw);
    let response = client
        .send_request(getrawtransaction_request)
        .expect("send_request failed");
    let getrawtransaction_response: GetRawTransactionRepsonse = response.result().unwrap();
    getrawtransaction_response
}

#[derive(Serialize, Deserialize, Debug)]
struct Vin {
    // TODO: Most vins don't have a coinbase key, so how can I make Vin types based on this?
    coinbase: Option<String>,
    txinwitness: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ScriptPubKey {
    asm: String,
    hex: String,
    address: Option<String>,
    #[serde(rename = "type")]
    type_: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Vout {
    value: f64,
    n: i64,
    scriptPubKey: ScriptPubKey,
}
#[derive(Serialize, Deserialize, Debug)]
struct DecodeRawTransactionResponse {
    txid: String,
    hash: String,
    version: i64,
    size: i64,
    weight: i64,
    locktime: i64,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
}
fn decode_raw_transaction(client: &Client, tx_hash: &Hash) -> DecodeRawTransactionResponse {
    let raw = &[to_raw_value(&tx_hash).unwrap()];
    let decode_raw_transaction_request = client.build_request("decoderawtransaction", raw);
    let response = client
        .send_request(decode_raw_transaction_request)
        .expect("send_request failed");
    let decode_raw_transaction_response: DecodeRawTransactionResponse = response.result().unwrap();
    decode_raw_transaction_response
}

// Demonstrate an example JSON-RCP call against bitcoind.
fn main() {
    let password = env::var("BITCOIND_PASSWORD").expect("BITCOIND_PASSWORD env variable not set");
    let username = env::var("BITCOIND_USERNAME").expect("BITCOIND_USERNAME env variable not set");
    let client = client("127.0.0.1:8332", &username, &password).expect("failed to create client");
    let blockcount = get_block_count(&client);
    println!("{:?}", blockcount);
    let mininginfo = get_mining_info(&client);
    println!("{:?}", mininginfo);
    let blockhash = get_block_hash(&client, &blockcount);
    println!("{:?}", blockhash);
    let block = get_block(&client, blockhash);
    let mut total_coins: f64 = 0.0;
    let mut count: i64 = 0;
    let mut blockhash = block.previousblockhash.clone();
    while let Some(previousblockhash) = blockhash {
        let first_transaction = &block.tx[0];
        let transaction_hash = get_raw_transaction(&client, first_transaction);
        let transaction = decode_raw_transaction(&client, &transaction_hash);
        let block = get_block(&client, previousblockhash);
        total_coins = total_coins + transaction.vout[0].value;
        blockhash = block.previousblockhash;
        count = count + 1;
        println!("{:?}: {:?}", count, total_coins);
    }
}
