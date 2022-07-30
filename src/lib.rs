mod command;
pub use command::{
    CallableCommand, GetBestBlockHashCommand, GetBlockCommand, GetBlockCommandVerbosity,
};
#[derive(Debug)]
pub struct BlockhashHexEncoded(pub String);
#[derive(Debug)]
pub struct Blockhash(pub String);
