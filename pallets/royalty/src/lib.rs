#![cfg_attr(not(feature = "std"), no_std)]

/// This is a simple implementation of EIP-2981
/// Since EIP-2981 is an extension to ERC-721,
/// I have also implemented some basic functionalities of ERC-721
/// https://eips.ethereum.org/EIPS/eip-2981
use frame_support::pallet_prelude::*;

use sp_std::vec::Vec;

pub use pallet::*;



#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
	pub struct Token<T: Config> {
		pub id: u32,
		pub owner: T::AccountId,
		pub royalty_precentage: u16,
		pub royalty_receiver: T::AccountId,
		pub uri: Vec<u8>, // for usage of text
	}


	#[pallet::config]
	pub trait Config: frame_system::Config {

	}

}
