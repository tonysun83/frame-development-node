#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Vec};
use frame_system::ensure_signed;
use frame_support::codec::{Encode, Decode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Debug, Default, Clone, PartialEq, Eq)]
pub struct Voter {
    name: Vec<u8>,
    president: Vec<u8>,
}


pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as PresidentVoters {
		PresidentVoters get(fn get_voters):
			map hasher(blake2_128_concat) T::AccountId => Voter;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		VoterAdded(Voter, AccountId),
	}
);


decl_error! {
	pub enum Error for Module<T: Trait> {
		NoneValue,
		StorageOverflow,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10000]
		fn add_voter(origin, name: Vec<u8>, president: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let new_voter = Voter{
				name,
				president,
			};
			<PresidentVoters<T>>::insert(&sender, new_voter.clone());
			Self::deposit_event(RawEvent::VoterAdded(new_voter, sender));
			Ok(())
		}
	}
}