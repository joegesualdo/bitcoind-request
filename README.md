# Bitcoind Request

> Send RPC commands to a bitcoind server.

---

**⚠️ This is experimental. Please use at your own risk.⚠️**

---

This library provides typesafe functions over raw RPC bitcoind commands to easily and safely retrieve information from a bitcoin node.

As this library only provides typesafety over raw RPC commands, functions will be mapped 1:1 to RPC commands. See [bitcoin core docs](https://bitcoincore.org/en/doc/0.17.0/rpc/) for a list of all commands and what they return.

If you're looking for additional information about the Bitcoin Network (i.e. time since last block, etc) , take a look at [bitcoin-node-query](https://github.com/joegesualdo/bitcoin-node-query), which provides additional functions to query bitcoin network data.

## Install

> Add package to Cargo.toml file

```rust
[dependencies]
bitcoind-request = "0.1.16"
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

// Create a Client.
let bitcoind_password: &str = ...
let bitcoind_username: &str = ...
let bitcoind_url = "127.0.0.1:8332"
let client = Client::new(
        &bitcoind_url,
        &bitcoind_username,
        &bitcoind_password
    ).expect("failed to create client");

// Get the estimated size of the block and undo files on disk.
// Note: this calls "getblockchaininfo" bitcoin core rpc command under the hood.
let blockchain_info = GetBlockchainInfoCommand::new().call(client);
println!("{}", blockchain_info.size_on_disk);

// Compute statistics about the total number and rate of transactions in the chain.
// Note: this calls "getchaintxstats" bitcoin core rpc command under the hood.
let maybe_chain_tx_stats = GetChainTxStatsCommand::new()
	.set_n_blocks(2016)
	.call(client);
println!("{:#?}", maybe_chain_tx_stats.unwrap());

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
