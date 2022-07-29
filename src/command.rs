/*
= Blockchain ==
getbestblockhash
getblock "blockhash" ( verbosity )
getblockchaininfo
getblockcount
getblockfilter "blockhash" ( "filtertype" )
getblockhash height
getblockheader "blockhash" ( verbose )
getblockstats hash_or_height ( stats )
getchaintips
getchaintxstats ( nblocks "blockhash" )
getdifficulty
getmempoolancestors "txid" ( verbose )
getmempooldescendants "txid" ( verbose )
getmempoolentry "txid"
getmempoolinfo
getrawmempool ( verbose mempool_sequence )
gettxout "txid" n ( include_mempool )
gettxoutproof ["txid",...] ( "blockhash" )
gettxoutsetinfo ( "hash_type" hash_or_height use_index )
preciousblock "blockhash"
pruneblockchain height
savemempool
scantxoutset "action" ( [scanobjects,...] )
verifychain ( checklevel nblocks )
verifytxoutproof "proof"

== Control ==
getmemoryinfo ( "mode" )
getrpcinfo
help ( "command" )
logging ( ["include_category",...] ["exclude_category",...] )
stop
uptime

== Generating ==
generateblock "output" ["rawtx/txid",...]
generatetoaddress nblocks "address" ( maxtries )
generatetodescriptor num_blocks "descriptor" ( maxtries )

== Mining ==
getblocktemplate ( "template_request" )
getmininginfo
getnetworkhashps ( nblocks height )
prioritisetransaction "txid" ( dummy ) fee_delta
submitblock "hexdata" ( "dummy" )
submitheader "hexdata"

== Network ==
addnode "node" "command"
clearbanned
disconnectnode ( "address" nodeid )
getaddednodeinfo ( "node" )
getconnectioncount
getnettotals
getnetworkinfo
getnodeaddresses ( count "network" )
getpeerinfo
listbanned
ping
setban "subnet" "command" ( bantime absolute )
setnetworkactive state

== Rawtransactions ==
analyzepsbt "psbt"
combinepsbt ["psbt",...]
combinerawtransaction ["hexstring",...]
converttopsbt "hexstring" ( permitsigdata iswitness )
createpsbt [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount,...},{"data":"hex"},...] ( locktime replaceable )
createrawtransaction [{"txid":"hex","vout":n,"sequence":n},...] [{"address":amount,...},{"data":"hex"},...] ( locktime replaceable )
decodepsbt "psbt"
decoderawtransaction "hexstring" ( iswitness )
decodescript "hexstring"
finalizepsbt "psbt" ( extract )
fundrawtransaction "hexstring" ( options iswitness )
getrawtransaction "txid" ( verbose "blockhash" )
joinpsbts ["psbt",...]
sendrawtransaction "hexstring" ( maxfeerate )
signrawtransactionwithkey "hexstring" ["privatekey",...] ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )
testmempoolaccept ["rawtx",...] ( maxfeerate )
utxoupdatepsbt "psbt" ( ["",{"desc":"str","range":n or [n,n]},...] )

== Signer ==
enumeratesigners

== Util ==
createmultisig nrequired ["key",...] ( "address_type" )
deriveaddresses "descriptor" ( range )
estimatesmartfee conf_target ( "estimate_mode" )
getdescriptorinfo "descriptor"
getindexinfo ( "index_name" )
signmessagewithprivkey "privkey" "message"
validateaddress "address"
verifymessage "address" "signature" "message"

== Wallet ==
abandontransaction "txid"
abortrescan
addmultisigaddress nrequired ["key",...] ( "label" "address_type" )
backupwallet "destination"
bumpfee "txid" ( options )
createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )
dumpprivkey "address"
dumpwallet "filename"
encryptwallet "passphrase"
getaddressesbylabel "label"
getaddressinfo "address"
getbalance ( "dummy" minconf include_watchonly avoid_reuse )
getbalances
getnewaddress ( "label" "address_type" )
getrawchangeaddress ( "address_type" )
getreceivedbyaddress "address" ( minconf )
getreceivedbylabel "label" ( minconf )
gettransaction "txid" ( include_watchonly verbose )
getunconfirmedbalance
getwalletinfo
importaddress "address" ( "label" rescan p2sh )
importdescriptors "requests"
importmulti "requests" ( "options" )
importprivkey "privkey" ( "label" rescan )
importprunedfunds "rawtransaction" "txoutproof"
importpubkey "pubkey" ( "label" rescan )
importwallet "filename"
keypoolrefill ( newsize )
listaddressgroupings
listdescriptors
listlabels ( "purpose" )
listlockunspent
listreceivedbyaddress ( minconf include_empty include_watchonly "address_filter" )
listreceivedbylabel ( minconf include_empty include_watchonly )
listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )
listtransactions ( "label" count skip include_watchonly )
listunspent ( minconf maxconf ["address",...] include_unsafe query_options )
listwalletdir
listwallets
loadwallet "filename" ( load_on_startup )
lockunspent unlock ( [{"txid":"hex","vout":n},...] )
psbtbumpfee "txid" ( options )
removeprunedfunds "txid"
rescanblockchain ( start_height stop_height )
send [{"address":amount,...},{"data":"hex"},...] ( conf_target "estimate_mode" fee_rate options )
sendmany "" {"address":amount,...} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode" fee_rate verbose )
sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode" avoid_reuse fee_rate verbose )
sethdseed ( newkeypool "seed" )
setlabel "address" "label"
settxfee amount
setwalletflag "flag" ( value )
signmessage "address" "message"
signrawtransactionwithwallet "hexstring" ( [{"txid":"hex","vout":n,"scriptPubKey":"hex","redeemScript":"hex","witnessScript":"hex","amount":amount},...] "sighashtype" )
unloadwallet ( "wallet_name" load_on_startup )
upgradewallet ( version )
walletcreatefundedpsbt ( [{"txid":"hex","vout":n,"sequence":n},...] ) [{"address":amount,...},{"data":"hex"},...] ( locktime options bip32derivs )
walletdisplayaddress bitcoin address to display
walletlock
walletpassphrase "passphrase" timeout
walletpassphrasechange "oldpassphrase" "newpassphrase"
walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )

== Zmq ==
getzmqnotifications
*/

use jsonrpc::Client;
use serde::{Deserialize, Serialize};
use serde_json::value::{to_raw_value, RawValue};

use crate::request;

pub struct BlockhashHexEncoded(pub String);
pub struct Blockhash(pub String);

pub enum Command {
    GetBestBlockHash(GetBestBlockHashCommand),
    GetBlock(GetBlockCommand),
}

pub trait CallableCommand {
    type Response;
    fn call(&self, client: &Client) -> Self::Response;
}

pub struct GetBestBlockHashCommand {
    blockhash_hex_encoded: BlockhashHexEncoded,
}
impl GetBestBlockHashCommand {
    pub fn new(blockhash_hex_encoded: BlockhashHexEncoded) -> Self {
        GetBestBlockHashCommand {
            blockhash_hex_encoded,
        }
    }
}
// TODO: struct GetBestBlockHashCommandResponse(String);
type GetBestBlockHashCommandResponse = String;

impl CallableCommand for GetBestBlockHashCommand {
    type Response = GetBestBlockHashCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let blockhash_arg = &self.blockhash_hex_encoded.0;
        let command = "getbestblockhash";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetBestBlockHashCommandResponse = r.result().unwrap();
        response
    }
}
enum GetBlockCommandVerbosity {
    SerializedHexEncodedData,                 // argument of 0
    BlockObjectWithoutTransactionInformation, // argument of 1
    BlockObjectWithTransactionInformation,    // argument of 2
}

pub struct GetBlockCommand {
    blockhash: Blockhash,
    verbosity: GetBlockCommandVerbosity,
}
impl GetBlockCommand {
    pub fn new(blockhash: Blockhash) -> Self {
        GetBlockCommand {
            blockhash,
            verbosity: GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation,
        }
    }
}
// TODO: Imlement a Block struct, that has better types.
//       For example, use Blockhash for 'hash' field.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetBlockCommandResponse {
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
    pub tx: Vec<String>,
}

// TODO: This will only work for GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation
//       because the json response has a different structure it returns for each verbosity option.
//       For example, GetBlockCommandVerbosity::BlockObjectWithTransactionInformation will return
//       an array for 'tx' field with full transaction structure, instead of only hashes for the
//       transaction. To accomplish this, we need to figure out how to have serde handle
//       conditional responses and map them to appropriate structs.
impl CallableCommand for GetBlockCommand {
    type Response = GetBlockCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let verbosity_arg = match self.verbosity {
            GetBlockCommandVerbosity::SerializedHexEncodedData => 0,
            GetBlockCommandVerbosity::BlockObjectWithoutTransactionInformation => 1,
            GetBlockCommandVerbosity::BlockObjectWithTransactionInformation => 2,
        };
        let blockhash_arg = &self.blockhash.0;
        let blockhash_arg_raw_value = to_raw_value(&blockhash_arg).unwrap();
        let verbosity_arg_raw_value = to_raw_value(&verbosity_arg).unwrap();
        let command = "getblock";
        let params = vec![blockhash_arg_raw_value, verbosity_arg_raw_value];
        println!("{:?}", params);
        let r = request(client, command, params);
        let response: GetBlockCommandResponse = r.result().unwrap();
        response
    }
}
