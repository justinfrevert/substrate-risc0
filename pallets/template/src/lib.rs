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

mod risc0;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use risc0_zkvm::Receipt;
	use crate::risc0::get_seal;

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
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1000000)]
		#[pallet::call_index(0)]
		// Risc0 factors example
		pub fn send_factors_receipt(
			origin: OriginFor<T>,
		) -> DispatchResult {
			use sp_std::vec;
			ensure_signed(origin)?;

			let seal = get_seal();
			let receipt = Receipt::new(&[135, 1, 0, 0, 0, 0, 0, 0], &seal);
			let image_id: risc0_zkvm::sha::Digest = risc0_zkvm::sha::Digest::new([133937908, 3727778855, 2100795079, 4075041621, 3309653911, 1482515540, 3335215972, 449739275]);

			let verif_result  = receipt.verify(&image_id).expect("Verification failed possibly due to incorrect parameters");

			Ok(())
		}

	}
}
