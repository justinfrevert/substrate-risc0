#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		inherent::Vec,
		pallet_prelude::*,
		traits::{BalanceStatus, Currency, ReservableCurrency},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::string::String;
	use risc0_zkvm::{SegmentReceipt, Receipt};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Proof was successfully verified and will be stored
		ProofVerified,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proof did not pass verification
		ProofNotVerified,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An extrinsic which verifies proofs for programs, forming a trustless relationship for
		/// others to check the verification result
		#[pallet::call_index(2)]
		#[pallet::weight(25000000)]
		pub fn store_and_verify_proof(
			origin: OriginFor<T>,
			image_id: [u32; 8],
			mut receipt_bytes: Vec<u8>
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let receipt_json: String = Decode::decode(&mut &receipt_bytes[..]).unwrap();
			let receipt: Receipt = serde_json::from_str(&receipt_json).unwrap();

			receipt.verify(image_id).map_err(|_| Error::<T>::ProofNotVerified)?;

			Self::deposit_event(Event::<T>::ProofVerified);
			Ok(())
		}
	}
}
