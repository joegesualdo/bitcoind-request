/*
getpeerinfo

Returns data about each connected network node as a json array of objects.

Result:
[                                  (json array)
  {                                (json object)
    "id" : n,                      (numeric) Peer index
    "addr" : "str",                (string) (host:port) The IP address and port of the peer
    "addrbind" : "str",            (string) (ip:port) Bind address of the connection to the peer
    "addrlocal" : "str",           (string) (ip:port) Local address as reported by the peer
    "network" : "str",             (string) Network (ipv4, ipv6, or onion) the peer connected through
    "mapped_as" : n,               (numeric) The AS in the BGP route to the peer used for diversifying
                                   peer selection (only available if the asmap config flag is set)
    "services" : "hex",            (string) The services offered
    "servicesnames" : [            (json array) the services offered, in human-readable form
      "str",                       (string) the service name if it is recognised
      ...
    ],
    "relaytxes" : true|false,      (boolean) Whether peer has asked us to relay transactions to it
    "lastsend" : xxx,              (numeric) The UNIX epoch time of the last send
    "lastrecv" : xxx,              (numeric) The UNIX epoch time of the last receive
    "last_transaction" : xxx,      (numeric) The UNIX epoch time of the last valid transaction received from this peer
    "last_block" : xxx,            (numeric) The UNIX epoch time of the last block received from this peer
    "bytessent" : n,               (numeric) The total bytes sent
    "bytesrecv" : n,               (numeric) The total bytes received
    "conntime" : xxx,              (numeric) The UNIX epoch time of the connection
    "timeoffset" : n,              (numeric) The time offset in seconds
    "pingtime" : n,                (numeric) ping time (if available)
    "minping" : n,                 (numeric) minimum observed ping time (if any at all)
    "pingwait" : n,                (numeric) ping wait (if non-zero)
    "version" : n,                 (numeric) The peer version, such as 70001
    "subver" : "str",              (string) The string version
    "inbound" : true|false,        (boolean) Inbound (true) or Outbound (false)
    "addnode" : true|false,        (boolean) Whether connection was due to addnode/-connect or if it was an automatic/inbound connection
                                   (DEPRECATED, returned only if the config option -deprecatedrpc=getpeerinfo_addnode is passed)
    "connection_type" : "str",     (string) Type of connection:
                                   outbound-full-relay (default automatic connections),
                                   block-relay-only (does not relay transactions or addresses),
                                   inbound (initiated by the peer),
                                   manual (added via addnode RPC or -addnode/-connect configuration options),
                                   addr-fetch (short-lived automatic connection for soliciting addresses),
                                   feeler (short-lived automatic connection for testing addresses).
                                   Please note this output is unlikely to be stable in upcoming releases as we iterate to
                                   best capture connection behaviors.
    "startingheight" : n,          (numeric) The starting height (block) of the peer
    "banscore" : n,                (numeric) The ban score (DEPRECATED, returned only if config option -deprecatedrpc=banscore is passed)
    "synced_headers" : n,          (numeric) The last header we have in common with this peer
    "synced_blocks" : n,           (numeric) The last block we have in common with this peer
    "inflight" : [                 (json array)
      n,                           (numeric) The heights of blocks we're currently asking from this peer
      ...
    ],
    "whitelisted" : true|false,    (boolean, optional) Whether the peer is whitelisted with default permissions
                                   (DEPRECATED, returned only if config option -deprecatedrpc=whitelisted is passed)
    "permissions" : [              (json array) Any special permissions that have been granted to this peer
      "str",                       (string) bloomfilter (allow requesting BIP37 filtered blocks and transactions),
                                   noban (do not ban for misbehavior; implies download),
                                   forcerelay (relay transactions that are already in the mempool; implies relay),
                                   relay (relay even in -blocksonly mode, and unlimited transaction announcements),
                                   mempool (allow requesting BIP35 mempool contents),
                                   download (allow getheaders during IBD, no disconnect after maxuploadtarget limit),
                                   addr (responses to GETADDR avoid hitting the cache and contain random records with the most up-to-date info).

      ...
    ],
    "minfeefilter" : n,            (numeric) The minimum fee rate for transactions this peer accepts
    "bytessent_per_msg" : {        (json object)
      "msg" : n,                   (numeric) The total bytes sent aggregated by message type
                                   When a message type is not listed in this json object, the bytes sent are 0.
                                   Only known message types can appear as keys in the object.
      ...
    },
    "bytesrecv_per_msg" : {        (json object)
      "msg" : n                    (numeric) The total bytes received aggregated by message type
                                   When a message type is not listed in this json object, the bytes received are 0.
                                   Only known message types can appear as keys in the object and all bytes received
                                   of unknown message types are listed under '*other*'.
    }
  },
  ...
]

Examples:
> bitcoin-cli getpeerinfo
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getpeerinfo", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::command::CallableCommand;
use crate::{client::Client, command::request::request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const GET_PEER_INFO_COMMAND: &str = "getpeerinfo";

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectedNetworkNode {
    id: u64,                    // Peer index
    addr: String,               // (host:port) The IP address and port of the peer
    addrbind: String,           // (ip:port) Bind address of the connection to the peer
    addrlocal: Option<String>,  // (ip:port) Local address as reported by the peer
    network: String,            // Network (ipv4, ipv6, or onion) the peer connected through
    mapped_as: Option<u64>, // The AS in the BGP route to the peer used for diversifying peer selection (only available if the asmap config flag is set)
    services: String,       // (hex) The services offered
    servicesnames: Vec<String>, // (json array) the services offered, in human-readable form. Each element of the array is (string) the service name if it is recognised
    relaytxes: bool,            // Whether peer has asked us to relay transactions to it
    lastsend: u64,              // (unix) The UNIX epoch time of the last send
    lastrecv: u64,              // (unix) The UNIX epoch time of the last receive
    last_transaction: u64, // (unix) The UNIX epoch time of the last valid transaction received from this peer
    last_block: u64,       // (unix) The UNIX epoch time of the last block received from this peer
    bytessent: u64,        // The total bytes sent
    bytesrecv: u64,        // The total bytes received
    conntime: u64,         // (unix) The UNIX epoch time of the connection
    timeoffset: i64,       // The time offset in seconds
    pingtime: f64,         // ping time (if available)
    minping: f64,          // minimum observed ping time (if any at all)
    pingwait: Option<u64>, // ping wait (if non-zero)
    version: u64,          // The peer version, such as 70001
    subver: String,        // The string version
    inbound: bool,         // Inbound (true) or Outbound (false)
    addnode: Option<bool>, // Whether connection was due to addnode/-connect or if it was an automatic/inbound connection. (DEPRECATED, returned only if the config option -deprecatedrpc=getpeerinfo_addnode is passed)
    //
    // TODO: Make this an enum
    connection_type: String, // Type of connection:
    //outbound-full-relay (default automatic connections),
    //block-relay-only (does not relay transactions or addresses),
    //inbound (initiated by the peer),
    //manual (added via addnode RPC or -addnode/-connect configuration options),
    //addr-fetch (short-lived automatic connection for soliciting addresses),
    //feeler (short-lived automatic connection for testing addresses).
    //Please note this output is unlikely to be stable in upcoming releases as we iterate to
    //best capture connection behaviors.
    startingheight: u64,       // The starting height (block) of the peer
    banscore: Option<u64>, // The ban score (DEPRECATED, returned only if config option -deprecatedrpc=banscore is passed)
    synced_headers: i64,   // The last header we have in common with this peer
    synced_blocks: i64,    // The last block we have in common with this peer
    inflight: Vec<u64>, // (json array) each element of array is (numeric) The heights of blocks we're currently asking from this peer
    whitelisted: Option<bool>, // (boolean, optional) Whether the peer is whitelisted with default permissions
    // (DEPRECATED, returned only if config option -deprecatedrpc=whitelisted is passed)
    // TODO: Make this an enum
    permissions: Vec<String>, //(json array) Any special permissions that have been granted to this peer
    // (string) bloomfilter (allow requesting BIP37 filtered blocks and transactions),
    // noban (do not ban for misbehavior; implies download),
    // forcerelay (relay transactions that are already in the mempool; implies relay),
    // relay (relay even in -blocksonly mode, and unlimited transaction announcements),
    // mempool (allow requesting BIP35 mempool contents),
    // download (allow getheaders during IBD, no disconnect after maxuploadtarget limit),
    // addr (responses to GETADDR avoid hitting the cache and contain random records with the most up-to-date info).
    minfeefilter: f64, // The minimum fee rate for transactions this peer accepts
    bytessent_per_msg: HashMap<String, u64>, // The total bytes sent aggregated by message type When a message type is not listed in this json object, the bytes sent are 0. Only known message types can appear as keys in the object.
    bytesrecv_per_msg: HashMap<String, u64>, //  The total bytes received aggregated by message type
                                             //When a message type is not listed in this json object, the bytes received are 0.
                                             //Only known message types can appear as keys in the object and all bytes received
                                             //of unknown message types are listed under '*other*'.
}
pub struct GetPeerInfoCommand {}
impl GetPeerInfoCommand {
    pub fn new() -> Self {
        GetPeerInfoCommand {}
    }
}

// TODO: struct GetPeerInfoCommandResponse(String);
#[derive(Serialize, Deserialize, Debug)]
pub struct GetPeerInfoCommandResponse(pub Vec<ConnectedNetworkNode>);

impl CallableCommand for GetPeerInfoCommand {
    type Response = GetPeerInfoCommandResponse;
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error> {
        let params = vec![];
        let r = request(client, GET_PEER_INFO_COMMAND, params);
        let response: GetPeerInfoCommandResponse = r.result()?;
        Ok(response)
    }
}
