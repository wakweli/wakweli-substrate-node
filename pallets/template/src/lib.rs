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

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	// use codec::{Decode, Encode};
	// use sp_std::vec::Vec;
	// use crate::v1::Post;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// ----------------------------------------------
	#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo, MaxEncodedLen)]
	// pub struct Post<AccountId> {
	pub struct Post {
		pub amount: u128,
		pub order: u32,
		// pub title: Vec<u8>,
		// pub content: Vec<u8>,
		// pub author: AccountId,
	}
	// #[pallet::storage]
	// pub type Posts<T: Config> = StorageMap<_, Twox64Concat, u32, Post<T::AccountId>>;
	
	#[pallet::storage]
	#[pallet::getter(fn posts)]
	pub type Posts<T: Config> = StorageMap<_, Twox64Concat, u32, Post>;

	#[pallet::storage]
	#[pallet::getter(fn next_post)]
	pub type NextPost<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn amount)]
	pub type Amount<T> = StorageValue<_, u128>;

	// #[pallet::storage]
	// #[pallet::getter(fn post)]
	// pub type CurrentPost<T> = StorageValue<_, Post>;
	// ----------------------------------------------

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		AmountStored { amount: u128, who: T::AccountId },
		PostStored { who: T::AccountId },
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
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]	
		pub fn set_amount(origin: OriginFor<T>, amount: u128) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Amount<T>>::put(amount);

			// Emit an event.
			Self::deposit_event(Event::AmountStored { amount, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
		
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]	
		pub fn add_amount(origin: OriginFor<T>, amount: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let mut a :u128 = Amount::<T>::get().unwrap_or(0);
			a += amount;
			<Amount<T>>::put(a);

			Self::deposit_event(Event::AmountStored { amount, who });
			Ok(())
		}
		
		// #[pallet::call_index(4)]
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]	
		// pub fn set_current_post(origin: OriginFor<T>, amount: u128, order: u32) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;
		// 	<CurrentPost<T>>::put(Post {
		// 		amount,
		// 		order,
		// 	});

		// 	Self::deposit_event(Event::PostStored { who });
		// 	Ok(())
		// }
		
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]	
		pub fn push_post(origin: OriginFor<T>, amount: u128, order: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			let post_id = NextPost::<T>::get().unwrap_or(0);
			Posts::<T>::insert(post_id, Post {
				amount,
				order,
			});
			NextPost::<T>::put(post_id + 1);

			Self::deposit_event(Event::PostStored { who });
			Ok(())
		}

		// #[pallet::call_index(2)]
		// #[pallet::weight(0)]
		// pub fn post(origin: OriginFor<T>, title: Vec<u8>, content: Vec<u8>) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	let post_id = NextPost::<T>::get().unwrap_or(0);
		// 	Posts::<T>::insert(post_id, Post::<T::AccountId> {
		// 		title,
		// 		content,
		// 		author: who,
		// 	});

		// 	NextPost::<T>::put(post_id + 1);
		// 	Ok(())
		// }
	}

	
	// pub mod v1 {
	// 	use codec::{Decode, Encode};
	// 	use sp_std::vec::Vec;

	// 	#[derive(Encode, Decode, Default, Clone, PartialEq)]
	// 	pub struct Post<AccountId> {
	// 		pub title: Vec<u8>,
	// 		pub content: Vec<u8>,
	// 		pub author: AccountId,
	// 	}
	// }
}