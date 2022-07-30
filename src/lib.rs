mod command;
pub use command::{
    CallableCommand, GetBestBlockHashCommand, GetBlockCommand, GetBlockCommandVerbosity,
    GetBlockCountCommand,
};
#[derive(Debug)]
pub struct BlockhashHexEncoded(pub String);
#[derive(Debug)]
pub struct Blockhash(pub String);
