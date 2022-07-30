mod command;
pub use command::{
    CallableCommand, GetBestBlockHashCommand, GetBlockCommand, GetBlockCommandResponse,
    GetBlockCommandTransactionResponse, GetBlockCommandVerbosity, GetBlockCountCommand,
    GetBlockHashCommand, GetRawTransactionCommand, GetRawTransactionCommandResponse, Vin,
};
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct BlockhashHexEncoded(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockhash(pub String);