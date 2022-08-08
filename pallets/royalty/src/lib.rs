#![cfg_attr(not(feature = "std"), no_std)]

/// This is a simple implementation of EIP-2981
/// Since EIP-2981 is an extension to ERC-721,
/// I have also implemented some basic functionalities of ERC-721
/// https://eips.ethereum.org/EIPS/eip-2981
use frame_support::pallet_prelude::*;
use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded};
use sp_std::vec::Vec;

pub use pallet::*;



#[frame_support::pallet]
pub mod pallet {
	use super::*;

	// attr pallet::without_storage_info is required, without it
	// there will be an error `the trait MaxEncodedLen is not implemented for xxx`
	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Token<T: Config> {
		pub id: T::TokenId,
		pub owner: T::AccountId,
		pub royalty_precentage: u16,
		pub royalty_receiver: T::AccountId,
		pub uri: Vec<u8>, // for usage of text
	}


	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TokenId: Parameter + AtLeast32BitUnsigned + Bounded + Default + Copy;
	}


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// [Minter, TokenId, TokenURI]
		// TokenMinted(Token<T>), ultimately I prefer this, but could not get it to work
		TokenMinted(T::AccountId, T::TokenId, Vec<u8>),
		/// [from, to, TokenId]
		TokenTransferred(T::AccountId, T::AccountId, T::TokenId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// When passing empty string
		InvalidTokenURI,
		/// Transferring tokens not owned by signer
		NotOwner,
	}

	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	pub(super) type Tokens<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat, T::TokenId,
		Blake2_128Concat, T::AccountId,
		Token<T>, OptionQuery
	>;

	/// Stores the next Token Id.
	#[pallet::storage]
	#[pallet::getter(fn token_pointer)]
	pub type TokenPointer<T: Config> = StorageValue<_, T::TokenId, ValueQuery>;

}
