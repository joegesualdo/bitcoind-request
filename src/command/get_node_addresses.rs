/*
getnodeaddresses ( count )

Return known addresses which can potentially be used to find new nodes in the network

Arguments:
1. count    (numeric, optional, default=1) The maximum number of addresses to return. Specify 0 to return all known addresses.

Result:
[                         (json array)
  {                       (json object)
    "time" : xxx,         (numeric) The UNIX epoch time of when the node was last seen
    "services" : n,       (numeric) The services offered
    "address" : "str",    (string) The address of the node
    "port" : n            (numeric) The port of the node
  },
  ...
]

Examples:
> bitcoin-cli getnodeaddresses 8
> curl --user myusername --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getnodeaddresses", "params": [8]}' -H 'content-type: text/plain;' http://127.0.0.1:8332/
 */
use crate::command::CallableCommand;
use crate::{client::Client, command::request::request};
use serde::{Deserialize, Serialize};
use serde_json::value::to_raw_value;

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeAddress {
    pub time: u64,       // The UNIX epoch time of when the node was last seen
    pub services: u64,   // The services offered
    pub address: String, // The address of the node
    pub port: u64,       // The port of the node
    // TODO: Use enum
    pub network: String, // The network (ipv4, ipv6, onion, i2p) the node connected through
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetNodeAddressesCommandResponse(pub Vec<NodeAddress>);

pub enum CountArg {
    MaxAddresses(u64),
    AllAddresses,
}
pub enum NetworkArg {
    All,
    Ipv4,
    Ipv6,
    Onion,
    I2p,
}
pub struct GetNodeAddressesCommand {
    count: CountArg,
    network: NetworkArg,
}

impl GetNodeAddressesCommand {
    pub fn new() -> Self {
        GetNodeAddressesCommand {
            count: CountArg::MaxAddresses(1),
            network: NetworkArg::All,
        }
    }
    pub fn set_count(mut self, count: CountArg) -> Self {
        self.count = count;
        self
    }
    pub fn set_network(mut self, network: NetworkArg) -> Self {
        self.network = network;
        self
    }
}

impl CallableCommand for GetNodeAddressesCommand {
    type Response = GetNodeAddressesCommandResponse;
    fn call(&self, client: &Client) -> Self::Response {
        let count_arg = match &self.count {
            CountArg::MaxAddresses(count) => count,
            CountArg::AllAddresses => &0,
        };
        let maybe_network_arg = match &self.network {
            NetworkArg::All => None,
            NetworkArg::Ipv4 => Some("ipv4"),
            NetworkArg::Ipv6 => Some("ipv6"),
            NetworkArg::Onion => Some("onion"),
            NetworkArg::I2p => Some("i2p"),
        };
        let count_arg_raw_value = to_raw_value(count_arg).unwrap();

        let params = match maybe_network_arg {
            Some(network_arg) => {
                let network_arg_raw_value = to_raw_value(network_arg).unwrap();
                vec![count_arg_raw_value, network_arg_raw_value]
            }
            None => vec![count_arg_raw_value],
        };
        let command = "getnodeaddresses";
        let r = request(client, command, params);
        let response: GetNodeAddressesCommandResponse = r.result().unwrap();
        response
    }
}
