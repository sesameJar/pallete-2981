#![cfg_attr(not(feature = "std"), no_std)]

/// This is a simple implementation of EIP-2981
/// Since EIP-2981 is an extension to ERC-721,
/// I have also implemented some basic functionalities of ERC-721
/// https://eips.ethereum.org/EIPS/eip-2981
use frame_support::pallet_prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
	}
}
