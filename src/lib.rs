pub mod client;
pub mod command;
// pub use command;
use serde::{Deserialize, Serialize};
#[derive(Debug)]
pub struct BlockhashHexEncoded(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockhash(pub String);
