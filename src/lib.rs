mod command;
pub use command::{
    CallableCommand, GetBestBlockHashCommand, GetBlockCommand, GetBlockCommandResponse,
    GetBlockCommandTransactionResponse, GetBlockCommandVerbosity, GetBlockCountCommand,
    GetBlockHashCommand,
};
#[derive(Debug)]
pub struct BlockhashHexEncoded(pub String);
#[derive(Debug)]
pub struct Blockhash(pub String);
