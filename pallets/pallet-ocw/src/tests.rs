use frame_support::{assert_noop, assert_ok};

use crate::{Error, mock::*}; // has Event
use crate::*; // has Event
use crate::mock::ExternalityBuilder;
use crate::mock::Event; // so use this Event

#[test]
fn it_works_for_default_value() {
    let (mut t, _, _) = ExternalityBuilder::build();
    t.execute_with(|| {
        // Dispatch a signed extrinsic.
        let acct: <TestRuntime as frame_system::Config>::AccountId = Default::default();
        assert_ok!(OCWModule::do_something(Origin::signed(acct), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(OCWModule::something(), Some(42));
    } );
}


#[test]
fn correct_error_for_none_value() {
    let (mut t, _, _) = ExternalityBuilder::build();
    t.execute_with(|| {
        let acct: <TestRuntime as frame_system::Config>::AccountId = Default::default();
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
			OCWModule::cause_error(Origin::signed(acct)),
			Error::<TestRuntime>::NoneValue
		);
    });
}

#[test]
fn correct_error_for_value() {
    let (mut t, _, _) = ExternalityBuilder::build();
    t.execute_with(|| {
        let acct: <TestRuntime as frame_system::Config>::AccountId = Default::default();

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
        let acct: <TestRuntime as frame_system::Config>::AccountId = Default::default();
        assert_ok!(OCWModule::submit_price(
			Origin::signed(acct),
			num
		));
        // A number is inserted to <Numbers> vec
        assert_eq!(OCWModule::prices(), vec![num]);
        // An event is emitted
        assert_eq!(System::events().len() ,1 );

        // println!("{:?}", System::events());
        assert!(System::events()
            .iter()
            .any(|er| er.event == Event::pallet_ocw(crate::Event::NewPrice(num, acct))));

        // Insert another number
        let num2 = num * 2;
        assert_ok!(OCWModule::submit_price(
			Origin::signed(acct),
			num2
		));
        // A number is inserted to <Numbers> vec
        assert_eq!(OCWModule::prices(), vec![num, num2]);
    });
}
//
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

#[test]
fn it_aggregates_the_price() {
    let (mut t, _, _) = ExternalityBuilder::build();
    t.execute_with(|| {
        assert_eq!(OCWModule::average_price(), None);
        assert_ok!(OCWModule::submit_price(Origin::signed(Default::default()), 27));
        assert_eq!(OCWModule::average_price(), Some(27));
        assert_ok!(OCWModule::submit_price(Origin::signed(Default::default()), 43));
        assert_eq!(OCWModule::average_price(), Some(35));
    });
}

// TODO:: Need this test
// #[test]
// fn should_make_http_call_and_parse_result() {
//     let (offchain, state) = testing::TestOffchainExt::new();
//     let mut t = sp_io::TestExternalities::default();
//     t.register_extension(OffchainWorkerExt::new(offchain));
//
//     price_oracle_response(&mut state.write());
//
//     t.execute_with(|| {
//         // when
//         let price = Example::fetch_price().unwrap();
//         // then
//         assert_eq!(price, 15523);
//     });
// }

// #[test]
// fn should_submit_signed_transaction_on_chain() {
//     const PHRASE: &str = "news slush supreme milk chapter athlete soap sausage put clutch what kitten";
//
//     let (offchain, offchain_state) = testing::TestOffchainExt::new();
//     let (pool, pool_state) = testing::TestTransactionPoolExt::new();
//     let keystore = KeyStore::new();
//     SyncCryptoStore::sr25519_generate_new(
//         &keystore,
//         crate::crypto::Public::ID,
//         Some(&format!("{}/hunter1", PHRASE))
//     ).unwrap();
//
//
//     let mut t = sp_io::TestExternalities::default();
//     t.register_extension(OffchainWorkerExt::new(offchain));
//     t.register_extension(TransactionPoolExt::new(pool));
//     t.register_extension(KeystoreExt(Arc::new(keystore)));
//
//     price_oracle_response(&mut offchain_state.write());
//
//     t.execute_with(|| {
//         // when
//         Example::fetch_price_and_send_signed().unwrap();
//         // then
//         let tx = pool_state.write().transactions.pop().unwrap();
//         assert!(pool_state.read().transactions.is_empty());
//         let tx = Extrinsic::decode(&mut &*tx).unwrap();
//         assert_eq!(tx.signature.unwrap().0, 0);
//         assert_eq!(tx.call, Call::Example(crate::Call::submit_price(15523)));
//     });
// }