use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, sp_std};

use frame_support::sp_runtime::{MultiSignature, CryptoTypeId};
use frame_support::sp_runtime::app_crypto::{Public, Pair, sr25519, ed25519, Ss58Codec, CryptoTypePublicPair};
use frame_support::sp_runtime::traits::{Verify, IdentifyAccount};
use std::io::Read;
use frame_support::pallet_prelude::Encode;
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::AccountId32;

type Signature = MultiSignature;
type AccountPublic = <Signature as Verify>::Signer;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;


#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
	});
}

#[test]
fn test_account() {
	use seed_reader::*;

	new_test_ext().execute_with(|| {

		// let seed = "blur pioneer frown science banana impose avoid law act strategy have bronze//1//stash";
		// // let account_id = Public::Pair::from_string(seed, None)
		// // 	.expect("static values are valid; qed")
		// // 	.public();
		//
		// let account_id = sr25519::Pair::from_string(seed, None)
		// 	.expect("Seed error.").public().to_vec();
		// // let account_id = account_id.to_vec();
		// let account_id = sp_core::hexdisplay::HexDisplay::from(&account_id);
		// // if let CryptoTypePublicPair(CryptoTypeId(info), data) = account_id.clone() {
		// // 	let info = sp_std::str::from_utf8(&info);
		// // 	let data = sp_std::str::from_utf8(&data.as_slice());
		// // 	println!("info = {:?}, data = {:?}", info, data);
		// // }
		//
		// // let account_id = sp_std::str::from_utf8(account_id);
		// println!("account_id = {:?}", &account_id);

		let content = test_file_content() ;
		println!("content: = {:?}", content);

		let rawkey_list = extract_content(content.as_str());
		println!("rawkey_list = {:?} ", &rawkey_list);

		//
		let insert_key_list: Vec<(&str, &str, String)> = rawkey_list.iter().map(|x| {
			make_author_insert_key_params(*x)
		}).collect() ;
		//
		println!("insert_key_list = {:?}", insert_key_list);
	});
}


// fn make_author_insert_key_params<'a >(raw_data: (&'a str, &'a str)) -> (&'a str, &'a str, String) {
// 	let mut account_id: [u8; 32] ;
// 	// println!("raw_data.0 = {:?}", raw_data.0);
// 	if "gran" == raw_data.0 {
// 		account_id = extract_hex_of_public::<ed25519::Public>(raw_data.1);
// 	}else{
// 		account_id = extract_hex_of_public::<sr25519::Public>(raw_data.1);
// 	}
// 	//
// 	let account_id = sp_core::hexdisplay::HexDisplay::from(&account_id);
// 	// println!("account_id = {:?}", &account_id);
// 	(raw_data.0,raw_data.1,format!("0x{}", &account_id))
// }
//
// fn extract_hex_of_public<TPublic: Public>(raw_data: &str) -> [u8; 32]
// 	where
// 		AccountPublic: From<<TPublic::Pair as Pair>::Public>,
// 	//  MultiSigner: From<<<TPublic as CryptoType>::Pair as sp_core::Pair>::Public>
// {
// 	let account_id = TPublic::Pair::from_string(raw_data, None).expect("Seed error").public();
// 	let account_id: AccountId32 = AccountPublic::from(account_id).into_account();
// 	account_id.into()
// }
//
// fn extract_content(content: &str) -> Vec<(&str, &str)> {
// 	let content_split = content.split("\n");
// 	let line_list: Vec<&str> = content_split.collect();
// 	line_list.iter().map(|line| {
// 		let raw_split = line.split(":");
// 		let raw_list: Vec<&str> = raw_split.collect();
// 		assert_eq!(raw_list.len(), 2, "The format of the raw data line is incorrect.");
// 		(raw_list[0], raw_list[1])
//
// 	}).collect()
// }
//
// // file extract
// fn test_file_content() -> String {
// 	let mut file = std::fs::File::open("/Users/mac/work-files/coding/git-files/ke-fan/ares-chain/pallets/template/src/authority_raw.txt").unwrap();
// 	let mut contents = String::new();
// 	file.read_to_string(&mut contents).unwrap();
// 	contents
// }

// Generate a crypto pair from seed.
// pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
// 	TPublic::Pair::from_string(&format!("//{}", seed), None)
// 		.expect("static values are valid; qed")
// 		.public()
// }
//
// /// Generate an account ID from seed.
// pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
// 	where
// 		AccountPublic: From<<TPublic::Pair as Pair>::Public>,
// {
// 	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
// }