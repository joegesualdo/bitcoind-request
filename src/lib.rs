mod command;
pub use command::{CallableCommand, GetBestBlockHashCommand, GetBlockCommand};
#[derive(Debug)]
pub struct BlockhashHexEncoded(pub String);
#[derive(Debug)]
pub struct Blockhash(pub String);
