use crate::{mock::*};

use super::*;

#[test]
fn request_no_register() {
    new_test_ext().execute_with(|| {
        assert!(AresModule::initiate_request(Origin::signed(2), 1, vec![], vec![]).is_err());
    });
}

#[test]
fn aggregators_can_be_registered() {
    new_test_ext().execute_with(|| {
        assert!(AresModule::register_aggregator(Origin::signed(1), "btc/eth".into(), "alice".into(), "api".into()).is_ok());
        assert!(AresModule::unregister_aggregator(Origin::signed(1)).is_ok());
    });

    new_test_ext().execute_with(|| {
        assert!(AresModule::unregister_aggregator(Origin::signed(1)).is_err());
    });
}

#[test]
fn unknown_operator() {
    new_test_ext().execute_with(|| {
        assert!(AresModule::register_aggregator(Origin::signed(1),"btc/eth".into(),"alice".into(),"api".into()).is_ok(),);
        assert!(<Aggregators<Test>>::contains_key(1));
        assert!(AresModule::initiate_request(Origin::signed(1), 2, "btcusdt".into(), vec![]).is_err());
    });
}

#[test]
fn operator_no_register() {
    new_test_ext().execute_with(|| {
        assert!(AresModule::feed_result(Origin::signed(1), 0, 10).is_err());
    });
}

#[test]
fn callback_not_match_operator() {
    new_test_ext().execute_with(|| {
        assert!(AresModule::register_aggregator(Origin::signed(1), "btc/eth".into(), "alice".into(), "api".into()).is_ok());
        assert!(AresModule::initiate_request(Origin::signed(2), 1, "btcusdt".into(), vec![]).is_ok());
        assert!(AresModule::feed_result(Origin::signed(3), 0, 10).is_err());
    });
}

#[test]
pub fn on_finalize() {
    new_test_ext().execute_with(|| {
        assert!(AresModule::register_aggregator(Origin::signed(1), "btc/eth".into(), "alice".into(), "api".into()).is_ok());
        assert!(AresModule::initiate_request(Origin::signed(1), 1, "btcusdt".into(), vec![]).is_ok());
        // Request has been killed, too old
        assert!(AresModule::feed_result(Origin::signed(1), 0, 10).is_ok());
    });
}