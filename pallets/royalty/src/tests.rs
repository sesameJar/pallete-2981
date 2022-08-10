use crate::{mock::*, Token, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn can_mint() {
	new_test_ext().execute_with(|| {
		let token_uri = "uri1".as_bytes().to_vec();

		assert_ok!(RoyaltyModule::mint(Origin::signed(100), token_uri.clone(), 1000, 200));
		assert_eq!(RoyaltyModule::token_pointer(), 1);

		let token = Token{
			id: 0,
			owner: 100,
			royalty_receiver: 200,
			royalty_percentage: 1000,
			uri: token_uri.clone()
		};

		assert_eq!(RoyaltyModule::tokens(0, &100), Some(token.clone()));

		System::assert_last_event(Event::RoyaltyModule(crate::Event::<Test>::TokenMinted(100, 0, token_uri)));
	});
}

#[test]
fn can_retrieve_royalty_info() {
	new_test_ext().execute_with(|| {

		let token_uri = "uri1".as_bytes().to_vec();

		assert_ok!(RoyaltyModule::mint(Origin::signed(100), token_uri, 1000, 200));

		assert_noop!(RoyaltyModule::royalty_info(Origin::signed(102), 0, 101, 10000), Error::<Test>::InvalidTokenId);

		assert_noop!(RoyaltyModule::royalty_info(Origin::signed(102), 1, 100, 10000), Error::<Test>::InvalidTokenId);

		assert_ok!(RoyaltyModule::royalty_info(Origin::signed(102), 0, 100, 10000));

		System::assert_last_event(Event::RoyaltyModule(crate::Event::RoyaltyInfo(200, 1000)));

	});
}

fn can_transfer_token() {
	new_test_ext().execute_with(|| {

		let token_uri = "uri1".as_bytes().to_vec();

		assert_ok!(RoyaltyModule::mint(Origin::signed(100), token_uri, 1000, 200));

		assert_noop!(RoyaltyModule::transfer(Origin::signed(101), 200, 0), Error::<Test>::InvalidTokenId);

		assert_ok!(RoyaltyModule::transfer(Origin::signed(100), 200, 0));

		assert_eq!(RoyaltyModule::tokens(0, 200).unwrap().owner, 200);

		System::assert_last_event(Event::RoyaltyModule(crate::Event::TokenTransferred(100, 200, 0)));

	});
}
