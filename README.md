# Substrate Risc0
This repository contains example Substrate and RISC Zero interactions. The main interaction that this repository focuses on is using RISC Zero as a prover, and a Substrate pallet as the verifier. Proving Substrate transactions has no public examples as of yet.

## Getting started with Risc0
See [this tutorial](https://dev.risczero.com/api/bonsai/quickstart) for an introduction to the prover and verifier used here.

## Prover
The host cost runs the existing "hello world" example from Risc0. This component uses [Subxt](https://github.com/paritytech/subxt) to send transactions to the chain.

To run the prover: 
[Start the local node](https://github.com/substrate-developer-hub/substrate-node-template#getting-started), then:
```shell
cd provers/factors
cargo run
```

## Verifier
The pallet serves as the verifier, using the [risc0-zkvm ](https://docs.rs/risc0-zkvm/latest/risc0_zkvm/). This was derived from the Substrate template, so the pallet still shares the name of the boilerplate [template pallet](https://github.com/justinFrevert/substrate-risc0/tree/main/pallets/template). 

## Substrate and local development
If unfamiliar with Substrate and how to install Substrate dependencies and run Substrate-based nodes, please see the [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template).

Building the node:
```shell
cargo +nightly build --release
```

Run the node:
```shell
./target/release/node-template --dev
```