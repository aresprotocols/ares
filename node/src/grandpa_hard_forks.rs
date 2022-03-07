#![allow(dead_code)]
#![allow(unused_variables)]

use sp_finality_grandpa::{AuthorityId as GrandpaId, AuthorityWeight as GrandpaWeight, SetId};
pub type AuthorityList = Vec<(GrandpaId, GrandpaWeight)>;
use crate::services;
use hex_literal::hex;
use services::BlockNumber;
use sp_core::{crypto::UncheckedInto, H256};
use std::str::FromStr;

pub fn forks() -> Vec<(SetId, (H256, BlockNumber), AuthorityList)> {
	let authorities: AuthorityList = vec![
		(hex!["b200d0328d26f7cbb67223c179ab14a2152d7afb6689f07b618fda33695d5fd4"].unchecked_into(), 1),
		(hex!["3b7345bd36fb53c50be544a7c2847b9673984fa587af0c27108d3d464183e94f"].unchecked_into(), 1),
		(hex!["2ce72e098beb0bc8ed6c812099bed8c7c60ae8208c94abf4212d7fdeaf11bab3"].unchecked_into(), 1),
		(hex!["a16c71b78c13cbd73e09cc348be1e8521ec2ce4c2615d4f2cf0e8148ba454a05"].unchecked_into(), 1),
	];

	let set_id = 3;
	let block_number = 731885;
	let block_hash = "0x9b3e7f0c39eaa544e73b2a580b33521667e9c0064c13724bfa281c5052858a99";
	let block_hash =
		H256::from_str(block_hash).expect("hard fork hashes are static and they should be carefully defined; qed.");
	return vec![(set_id, (block_hash, block_number), authorities)]
}
