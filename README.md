Idiomatic interface to the Hermez v1 API for Rust.

The Hermez node API is the layer that allows 3rd party apps and services to interface with the node to use the layer two features of the Hermez rollup. Examples of these apps are:

 * Wallet: send L2 transactions, check balance, ...
 * Explorer: List transactions, slots, batches, ...
 * Exchange integrations

Note that some of the interactions with the rollup must be done using the Ethereum network directly. Another way to integrate with the rollup is to deploy a node and connect directly to its PostgreSQL database.

# Usage

All access is done though a `HermezApi` object. Create an instance by calling
HermezApi::new() with a valid base URL for the node.

HermezApi provides three URL constants that can be used.
* MAINNET_URL: Use with Mainnet
* TESTNET_URL: Use with the Rinkeby test net
* LOCALHOST_URL: Use with a locally hosted node. Use for unit tests.

# Examples

```
use hermez_api::HermezApi;

let api_test = HermezApi::new(HermezApi::TESTNET_URL).unwrap();
let api_other = HermezApi::new("https://some.other.domain/hermez").unwrap();
```

# Useful Links

[API Reference](https://apidoc.hermez.network/ "Hermez API Reference")

[Hermez Home](https://hermez.io)
