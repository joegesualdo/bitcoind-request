use bitcoin_request::{
    Blockhash, BlockhashHexEncoded, CallableCommand, GetBestBlockHashCommand, GetBlockCommand,
    GetBlockCommandVerbosity, GetBlockCountCommand,
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

fn main() {
    let password = env::var("BITCOIND_PASSWORD").expect("BITCOIND_PASSWORD env variable not set");
    let username = env::var("BITCOIND_USERNAME").expect("BITCOIND_USERNAME env variable not set");
    let client = client("127.0.0.1:8332", &username, &password).expect("failed to create client");
    let blockhash_hex_encoded = BlockhashHexEncoded(
        "00000000000000000007d1712328c3b95adc170e3e04b2499c04a4ee2905f72e".to_string(),
    );

    let best_block_hash = GetBestBlockHashCommand::new(blockhash_hex_encoded).call(&client);
    println!("{:?}", best_block_hash);

    let blockhash = Blockhash(best_block_hash);

    let response = GetBlockCommand::new(blockhash)
        .verbosity(GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation)
        .call(&client);
    println!("{:#?}", response);

    let block_count = GetBlockCountCommand::new().call(&client);
    println!("{:#?}", block_count);
}
