# Substrate Risc0
This repository contains an example Substrate chain I'm using to explore my curiosity on the potential of zero knowledge proof verification on-chain. Specifically, the chain contains a pallet which knows how to verify `Receipts` from Risc0 programs.

This might be a good starting point for those looking to experiment with Risc0 verification on Substrate chains.

## Getting started with Risc0
See [this tutorial](https://www.risczero.com/docs/examples/hello_multiply) for an introduction to the prover and verifier used here.

## Prover
The host cost runs the existing "factors" example from Risc0. This component uses [Subxt](https://github.com/paritytech/subxt) to send transactions to the chain.
Current examples:
- factors(risc0 hello world)
- wasm
## Verifier
The pallet serves as the verifier, using the [risc0-zkvm ](https://docs.rs/risc0-zkvm/latest/risc0_zkvm/). 
