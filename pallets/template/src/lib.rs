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

mod common;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use risc0_zkvm::Receipt;
	use crate::common::MULTIPLY_IMAGE_ID;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The seal was verified
		VerificationSuccess
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The seal could not be verified
		FailedVerification,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1000000)]
		#[pallet::call_index(0)]
		// Risc0 factors example
		pub fn send_factors_receipt(
			origin: OriginFor<T>,
			// Caution: in a real Substrate chain, you'd use `BoundedVec` for any of these large inputs
			journal: Vec<u8>,
			seal: Vec<u32>
		) -> DispatchResult {
			ensure_signed(origin)?;
			let receipt = Receipt::new(&journal, &seal);
			receipt.verify(&MULTIPLY_IMAGE_ID).map_err(|_| Error::<T>::FailedVerification)?;
			Self::deposit_event(Event::<T>::VerificationSuccess);
			Ok(())
		}
	}
}
