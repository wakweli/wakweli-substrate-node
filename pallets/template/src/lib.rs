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
	use codec::{Decode, Encode};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// ----------------------------------------------
	#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo)]
	pub struct Pool<AccountId> {
		pub name: Vec<u8>,
		pub description: Vec<u8>,
		pub amount: u128,
		pub certifier: AccountId,
	}

	#[pallet::storage]
	#[pallet::getter(fn pools)]
	pub type Pools<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Pool<T::AccountId>>;

	// #[pallet::storage]
	// #[pallet::getter(fn next_pool)]
	// pub type NextPool<T: Config> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn pool_stakes)]
	pub type PoolStakes<T: Config> =
		StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, T::AccountId, u128>;
	// ----------------------------------------------

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		PoolStored { who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_pool(
			origin: OriginFor<T>,
			name: Vec<u8>,
			description: Vec<u8>,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if Pools::<T>::contains_key(who.clone()) {
				return Err(Error::<T>::NoneValue.into());
			}

			Pools::<T>::insert(
				who.clone(),
				Pool { name, description, amount, certifier: who.clone() },
			);

			Self::deposit_event(Event::PoolStored { who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn stake_pool(
			origin: OriginFor<T>,
			certifier: T::AccountId,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// if the pool doesn't exist, return error
			if !Pools::<T>::contains_key(certifier.clone()) {
				return Err(Error::<T>::NoneValue.into());
			}

			// TODO: this line only stores the value it doesn't actually transfer the amount
			PoolStakes::<T>::set(certifier, who.clone(), Some(amount));

			Self::deposit_event(Event::PoolStored { who });
			Ok(())
		}
	}
}
