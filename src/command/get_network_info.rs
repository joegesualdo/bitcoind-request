/*
getnetworkinfo
Returns an object containing various state info regarding P2P networking.

Result:
{                                                    (json object)
  "version" : n,                                     (numeric) the server version
  "subversion" : "str",                              (string) the server subversion string
  "protocolversion" : n,                             (numeric) the protocol version
  "localservices" : "hex",                           (string) the services we offer to the network
  "localservicesnames" : [                           (json array) the services we offer to the network, in human-readable form
    "str",                                           (string) the service name
    ...
  ],
  "localrelay" : true|false,                         (boolean) true if transaction relay is requested from peers
  "timeoffset" : n,                                  (numeric) the time offset
  "connections" : n,                                 (numeric) the total number of connections
  "connections_in" : n,                              (numeric) the number of inbound connections
  "connections_out" : n,                             (numeric) the number of outbound connections
  "networkactive" : true|false,                      (boolean) whether p2p networking is enabled
  "networks" : [                                     (json array) information per network
    {                                                (json object)
      "name" : "str",                                (string) network (ipv4, ipv6, onion, i2p)
      "limited" : true|false,                        (boolean) is the network limited using -onlynet?
      "reachable" : true|false,                      (boolean) is the network reachable?
      "proxy" : "str",                               (string) ("host:port") the proxy that is used for this network, or empty if none
      "proxy_randomize_credentials" : true|false     (boolean) Whether randomized credentials are used
    },
    ...
  ],
  "relayfee" : n,                                    (numeric) minimum relay fee rate for transactions in BTC/kvB
  "incrementalfee" : n,                              (numeric) minimum fee rate increment for mempool limiting or BIP 125 replacement in BTC/kvB
  "localaddresses" : [                               (json array) list of local addresses
    {                                                (json object)
      "address" : "str",                             (string) network address
      "port" : n,                                    (numeric) network port
      "score" : n                                    (numeric) relative score
    },
    ...
  ],
  "warnings" : "str"                                 (string) any network and blockchain warnings
}

Examples:
> bitcoin-cli getnetworkinfo
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getnetworkinfo", "params": []}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
*/
use crate::client::Client;
use crate::command::request::request;
use crate::command::CallableCommand;
use serde::Deserialize;
use serde::Serialize;
use serde_json::value::RawValue;

pub struct GetNetworkInfoCommand {}
impl GetNetworkInfoCommand {
    pub fn new() -> Self {
        GetNetworkInfoCommand {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Network {
    pub name: String,
    pub limited: bool,
    pub reachable: bool,
    pub proxy: String,
    pub proxy_randomize_credentials: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LocalAddress {
    pub address: String,
    pub port: u64,
    pub score: u64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetNetworkInfoCommandResponse {
    pub version: u64,
    pub subversion: String,
    pub protocolversion: u64,
    pub localservices: String,
    pub localservicesnames: Vec<String>,
    pub localrelay: bool,
    pub timeoffset: u64,
    pub connections: u64,
    pub connections_in: u64,
    pub connections_out: u64,
    pub networkactive: bool,
    pub networks: Vec<Network>,
    pub relayfee: f64,
    pub incrementalfee: f64,
    pub localaddresses: Vec<LocalAddress>,
    pub warnings: String,
}

impl CallableCommand for GetNetworkInfoCommand {
    type Response = GetNetworkInfoCommandResponse;
    fn call(&self, client: &Client) -> Result<Self::Response, jsonrpc::Error> {
        let command = "getnetworkinfo";
        let params: Vec<Box<RawValue>> = vec![];
        let r = request(client, command, params);
        let response: GetNetworkInfoCommandResponse = r.result()?;
        Ok(response)
    }
}
