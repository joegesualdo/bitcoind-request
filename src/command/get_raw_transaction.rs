/*
getrawtransaction "txid" ( verbose "blockhash" )

Return the raw transaction data.

By default this function only works for mempool transactions. When called with a blockhash
argument, getrawtransaction will return the transaction if the specified block is available and
the transaction is found in that block. When called without a blockhash argument, getrawtransaction
will return the transaction if it is in the mempool, or if -txindex is enabled and the transaction
is in a block in the blockchain.

Hint: Use gettransaction for wallet transactions.

If verbose is 'true', returns an Object with information about 'txid'.
If verbose is 'false' or omitted, returns a string that is serialized, hex-encoded data for 'txid'.

Arguments:
1. txid         (string, required) The transaction id
2. verbose      (boolean, optional, default=false) If false, return a string, otherwise return a json object
3. blockhash    (string, optional) The block in which to look for the transaction

Result (if verbose is not set or set to false):
"str"    (string) The serialized, hex-encoded data for 'txid'

Result (if verbose is set to true):
{                                    (json object)
  "in_active_chain" : true|false,    (boolean) Whether specified block is in the active chain or not (only present with explicit "blockhash" argument)
  "hex" : "hex",                     (string) The serialized, hex-encoded data for 'txid'
  "txid" : "hex",                    (string) The transaction id (same as provided)
  "hash" : "hex",                    (string) The transaction hash (differs from txid for witness transactions)
  "size" : n,                        (numeric) The serialized transaction size
  "vsize" : n,                       (numeric) The virtual transaction size (differs from size for witness transactions)
  "weight" : n,                      (numeric) The transaction's weight (between vsize*4-3 and vsize*4)
  "version" : n,                     (numeric) The version
  "locktime" : xxx,                  (numeric) The lock time
  "vin" : [                          (json array)
    {                                (json object)
      "txid" : "hex",                (string) The transaction id
      "vout" : n,                    (numeric) The output number
      "scriptSig" : {                (json object) The script
        "asm" : "str",               (string) asm
        "hex" : "hex"                (string) hex
      },
      "sequence" : n,                (numeric) The script sequence number
      "txinwitness" : [              (json array)
        "hex",                       (string) hex-encoded witness data (if any)
        ...
      ]
    },
    ...
  ],
  "vout" : [                         (json array)
    {                                (json object)
      "value" : n,                   (numeric) The value in BTC
      "n" : n,                       (numeric) index
      "scriptPubKey" : {             (json object)
        "asm" : "str",               (string) the asm
        "hex" : "str",               (string) the hex
        "reqSigs" : n,               (numeric) The required sigs
        "type" : "str",              (string) The type, eg 'pubkeyhash'
        "addresses" : [              (json array)
          "str",                     (string) bitcoin address
          ...
        ]
      }
    },
    ...
  ],
  "blockhash" : "hex",               (string) the block hash
  "confirmations" : n,               (numeric) The confirmations
  "blocktime" : xxx,                 (numeric) The block time expressed in UNIX epoch time
  "time" : n                         (numeric) Same as "blocktime"
}

Examples:
> bitcoin-cli getrawtransaction "mytxid"
> bitcoin-cli getrawtransaction "mytxid" true
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getrawtransaction", "params": ["mytxid", true]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
> bitcoin-cli getrawtransaction "mytxid" false "myblockhash"
> bitcoin-cli getrawtransaction "mytxid" true "myblockhash"
*/
use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;

use crate::{command::CallableCommand, Blockhash};

use crate::command::request::request;

type TxId = String;
pub struct GetRawTransactionCommand {
    txid: TxId,
    is_verbose: bool,
    blockhash: Option<Blockhash>,
}

impl GetRawTransactionCommand {
    pub fn new(txid: TxId) -> Self {
        GetRawTransactionCommand {
            txid,
            is_verbose: false,
            blockhash: None,
        }
    }
    pub fn verbose(&mut self, verbose: bool) -> &Self {
        self.is_verbose = verbose;
        self
    }
    pub fn blockhash(&mut self, blockhash: Blockhash) -> &Self {
        self.blockhash = Some(blockhash);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HexEncodedWitnessData(pub String);
#[derive(Serialize, Deserialize, Debug)]
pub struct BitcoinAddress(pub String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPubKey {
    pub asm: String,           // "asm", NOT A HEX
    pub hex: String,           // the hex
    pub req_sigs: Option<u64>, // The required sigs
    #[serde(alias = "type")]
    pub type_: String, // The type, eg 'pubkeyhash'
                               //pub addresses: Vec<BitcoinAddress>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScriptSig {
    pub asm: String, // "asm", NOT A HEX
    pub hex: String, // "hex", hex
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Vin {
    Coinbase(CoinbaseVin),
    NonCoinbase(NonCoinbaseVin),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoinbaseVin {
    pub coinbase: String,
    pub sequence: u64,                           // The script sequence number
    pub txinwitness: Vec<HexEncodedWitnessData>, // hex-encoded witness data (if any)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NonCoinbaseVin {
    pub txid: String, // "hex" The transaction id
    pub vout: u64,    // The output number
    pub script_sig: ScriptSig,
    pub sequence: u64, // The script sequence number
    // TODO: Why is this optional?
    pub txinwitness: Option<Vec<HexEncodedWitnessData>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: f64, // The value in BTC
    pub n: u64,     // index
    pub script_pub_key: ScriptPubKey,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hex: String,   // "hex" The serialized, hex-encoded data for 'txid'
    pub txid: String,  // "hex" The transaction id (same as provided)
    pub hash: String,  // "hex" The transaction hash (differs from txid for witness transactions)
    pub size: u64,     // The serialized transaction size
    pub vsize: u64,    // The virtual transaction size (differs from size for witness transactions)
    pub weight: u64,   //  The transaction's weight (between vsize*4-3 and vsize*4)
    pub version: u64,  //  The version
    pub locktime: u64, // The lock time
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub blockhash: String,  // "hex" the block hash
    pub confirmations: u64, // "hex" The confirmations
    pub blocktime: u64,     // "unix time" The block time expressed in UNIX epoch time
    pub time: u64,          // "unix time" Same as "blocktime"
}

// TODO: I don't think this belongs in this package. We should focus on RPC request and responses
// and abstract a better data layer into another package.
impl Transaction {
    pub fn is_coinbase_transaction(&self) -> bool {
        match self.vin.first().unwrap() {
            Vin::Coinbase(_x) => true,
            Vin::NonCoinbase(_x) => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GetRawTransactionCommandResponse {
    SerializedHexEncodedData(String),
    Transaction(Transaction),
}

// TODO: This will only work for GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation
//       because the json response has a different structure it returns for each verbosity option.
//       For example, GetBlockCommandVerbosity::BlockObjectWithTransactionInformation will return
//       an array for 'tx' field with full transaction structure, instead of only hashes for the
//       transaction. To accomplish this, we need to figure out how to have serde handle
//       conditional responses and map them to appropriate structs.
impl CallableCommand for GetRawTransactionCommand {
    type Response = GetRawTransactionCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let txid_arg = &self.txid;
        let verbose_arg = &self.is_verbose;
        // TODO: Add blockhas param!
        //let blockhash_arg = &self.blockhash.0;
        let txid_arg_raw_value = to_raw_value(&txid_arg).unwrap();
        let verbose_arg_raw_value = to_raw_value(&verbose_arg).unwrap();
        let command = "getrawtransaction";
        let params = vec![txid_arg_raw_value, verbose_arg_raw_value];
        let r = request(client, command, params);
        let response: GetRawTransactionCommandResponse = r.result().unwrap();
        response
    }
}
