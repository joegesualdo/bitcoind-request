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
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetNodeAddressesCommandResponse(pub Vec<NodeAddress>);

pub enum CountArg {
    MaxAddresses(u64),
    AllAddresses,
}
pub struct GetNodeAddressesCommand {
    count: CountArg,
}
impl GetNodeAddressesCommand {
    pub fn new() -> Self {
        GetNodeAddressesCommand {
            count: CountArg::MaxAddresses(1),
        }
    }
    pub fn set_count(mut self, count: CountArg) -> Self {
        self.count = count;
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
        let count_arg_raw_value = to_raw_value(count_arg).unwrap();
        let command = "getnodeaddresses";
        let params = vec![count_arg_raw_value];
        let r = request(client, command, params);
        let response: GetNodeAddressesCommandResponse = r.result().unwrap();
        response
    }
}
