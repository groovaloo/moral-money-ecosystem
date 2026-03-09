#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[pallet::pallet]
pub struct Pallet<T>(_);

#[pallet::config]
pub trait Config: frame_system::Config {}

#[pallet::storage]
#[pallet::getter(fn reputation)]

pub type Reputation<T: Config> =
StorageMap<
_,
Blake2_128Concat,
T::AccountId,
u32,
ValueQuery
>;

#[pallet::call]
impl<T: Config> Pallet<T> {

#[pallet::weight(0)]

pub fn increase_reputation(
origin: OriginFor<T>,
amount: u32
) -> DispatchResult {

let who = ensure_signed(origin)?;

Reputation::<T>::mutate(&who, |rep| {
*rep += amount;
});

Ok(())

}

}

}
