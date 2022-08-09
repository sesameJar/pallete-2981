#![cfg_attr(not(feature = "std"), no_std)]

/// This is a simple implementation of EIP-2981
/// Since EIP-2981 is an extension to ERC-721,
/// I have also implemented some basic functionalities of ERC-721
/// https://eips.ethereum.org/EIPS/eip-2981
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use sp_runtime::{ArithmeticError, traits::{AtLeast32BitUnsigned, Bounded, One, CheckedAdd}};
use sp_std::vec::Vec;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


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
		pub royalty_percentage: u16,
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
		NotExistingToken,
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


	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// Mint a new Token
		#[pallet::weight(1000)]
		pub fn mint(
			origin: OriginFor<T>,
			token_uri: Vec<u8>,
			royalty_percentage: u16,
			royalty_receiver: T::AccountId
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let token_id = Self::next_token_id()?;
			let token = Token {
				id: token_id,
				owner: sender.clone(),
				royalty_receiver,
				royalty_percentage,
				uri: token_uri.clone()
			};

			// mint and save token in the Tokens mapping
			Tokens::<T>::insert(token_id, &sender, &token);
			// Emit event
			Self::deposit_event(Event::TokenMinted(sender, token_id, token_uri));

			Ok(())
		}


		/// Calculate the amount royalty_receiver is supposed to receive from the given sale_amount
		/// ultimately I want this function to be unsigned like an RPC call but could not figure it out on time
		/// To check the result one should check for the proper event
		#[pallet::weight(1000)]
		pub fn royalty_info(
			origin: OriginFor<T>,
			token_id: T::TokenId,
			sale_amount: u32
		) -> DispatchResult {
			let _sender = ensure_signed(origin)?;


			Ok(())
		}

		/// Transfers the token
		#[pallet::weight(1000)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			token_id: T::TokenId
		) -> DispatchResult {
			let _sender = ensure_signed(origin)?;


			Ok(())
		}
	}

}

impl<T: Config> Pallet<T> {
	fn next_token_id() -> Result<T::TokenId, DispatchError> {
		TokenPointer::<T>::try_mutate(|next_token_id| -> Result<T::TokenId, DispatchError> {
			let current_token_id = *next_token_id;

			// add 1 and check for oveflow
			*next_token_id = next_token_id.checked_add(&One::one()).ok_or(ArithmeticError::Overflow)?;
			Ok(current_token_id)
		} )
	}
}
