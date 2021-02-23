use frame_support::{assert_noop, assert_ok};

use crate::{Error, mock::*};
use crate::*;
use crate::mock::ExternalityBuilder;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        let acct: <Test as frame_system::Trait>::AccountId = Default::default();
        assert_ok!(OCWModule::do_something(Origin::signed(acct), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(OCWModule::something(), Some(42));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        let acct: <Test as frame_system::Trait>::AccountId = Default::default();
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
			OCWModule::cause_error(Origin::signed(acct)),
			Error::<Test>::NoneValue
		);
    });
}

#[test]
fn correct_error_for_value() {
    new_test_ext().execute_with(|| {
        let acct: <Test as frame_system::Trait>::AccountId = Default::default();

        // Dispatch a signed extrinsic.
        assert_ok!(OCWModule::do_something(Origin::signed(acct), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(OCWModule::something(), Some(42));

        assert_ok!(OCWModule::cause_error(Origin::signed(acct)));
    });
}

#[test]
fn add_price_signed_works() {
    let (mut t, _, _) = ExternalityBuilder::build();
    t.execute_with(|| {
        // call submit_number_signed
        let num = 32;
        let acct: <Test as frame_system::Trait>::AccountId = Default::default();
        assert_ok!(OCWModule::submit_price(
			Origin::signed(acct),
			num
		));
        // A number is inserted to <Numbers> vec
        assert_eq!(<Prices>::get(), vec![num]);
        // An event is emitted
        assert!(System::events()
            .iter()
            .any(|er| er.event == TestEvent::pallet_ocw(RawEvent::NewPrice(num, acct))));

        // Insert another number
        let num2 = num * 2;
        assert_ok!(OCWModule::submit_price(
			Origin::signed(acct),
			num2
		));
        // A number is inserted to <Numbers> vec
        assert_eq!(<Prices>::get(), vec![num, num2]);
    });
}

#[test]
fn parse_price_works() {
    let test_data = vec![
        ("{\"msg\":\"success\",\"code\":0,\"data\":{\"market\":null,\"symbol\":\"btcusdt\",\"price\":23383.08,\"nodes\":null,\"sn\":null,\"systs\":1608654228412,\"ts\":1608654228412}}",
         Some(2338308)),
    ];

    for (json, expected) in test_data {
        assert_eq!(expected, OCWModule::parse_price(json));
    }
}