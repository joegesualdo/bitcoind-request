☠️⚠️ Work In Progress ⚠️☠️
# Bitcoind Request 
> Send RPC commands to a bitcoind server.

This library provides typesafe functions over raw RPC bitcoind commands to easily and safely retrieve information from a bitcoin node.


## Install
> Add package to Cargo.toml file
```rust
[dependencies]
bitcoind-request = "0.1.3"
```

## Usage:
```rust
use bitcoind_request::{
    client::Client,
    command::{
        get_blockchain_info::GetBlockchainInfoCommand,
        CallableCommand,
    },
};

let bitcoind_password: &str = ...
let bitcoind_username: &str = ...
let bitcoind_url = "127.0.0.1:8332"
let client = Client::new(
        &bitcoind_url,
        &bitcoind_username,
        &bitcoind_password
    ).expect("failed to create client");

let blockchain_info = GetBlockchainInfoCommand::new().call(client);
println!("{}", blockchain_info.size_on_disk)

let chain_tx_stats = GetChainTxStatsCommand::new()
	.set_n_blocks(2016)
	.call(client);
```
## Commands
List of all bitcoind commands can be found at [bitcoin.org](https://bitcoincore.org/en/doc/0.21.0/rpc/)

They can also be found in the bitcoin-cli docs:
```zsh
$ bitcoin-cli help
```

## Related
- [bitcoin-node-query](https://github.com/joegesualdo/bitcoin-node-query) - Query Bitcoin Node for information
- [bitcoin-terminal-dashboard](https://github.com/joegesualdo/bitcoin-terminal-dashboard) - Bitcoin Dashboard in the terminal

## License
MIT © [Joe Gesualdo]()
