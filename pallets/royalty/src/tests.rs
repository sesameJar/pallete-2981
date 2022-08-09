use crate::{mock::*, Token};
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

		// System::assert_last_event(Event::RoyaltyModule(crate::Event::<Test>::TokenMinted(100, 0, token_uri)));
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
