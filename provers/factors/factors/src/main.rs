// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkvm::Prover;

use subxt::{
	ext::{
		sp_core::{sr25519::Pair as SubxtPair, Pair},
	},
	tx::PairSigner,
	OnlineClient, PolkadotConfig,
};

// // Runtime types, etc
#[subxt::subxt(runtime_metadata_path = "./metadata.scale")]
pub mod myexamplenode {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Pick two numbers
    let a: u64 = 17;
    let b: u64 = 23;

    println!("Creating Proof");

    // Multiply them inside the ZKP
    // First, we make the prover, loading the 'multiply' method
    let mut prover = Prover::new(MULTIPLY_ELF).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );
    println!("Ading input to proof");

    // Next we send a & b to the guest
    prover.add_input_u32_slice(&to_vec(&a).expect("should be serializable"));
    println!("Ading input to proof");
    prover.add_input_u32_slice(&to_vec(&b).expect("should be serializable"));
    println!("Running prover...");
    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Valid code should be provable if it doesn't overflow the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");
    println!("Got proof");

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.journal).expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // // Verify receipt, panic if it's wrong
    receipt.verify(&MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    let api = OnlineClient::<PolkadotConfig>::new().await.unwrap();

    let restored_key = SubxtPair::from_string("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a", None).unwrap();
    let signer = PairSigner::new(restored_key);

    api
        .tx()
        .sign_and_submit_then_watch_default(
            &myexamplenode::tx().template_module().send_factors_receipt(
                // receipt.journal,
                receipt.seal,
                // MULTIPLY_ID
            ),
            &signer
        )
        .await?
        .wait_for_finalized()
        .await?;
    Ok(())
}
