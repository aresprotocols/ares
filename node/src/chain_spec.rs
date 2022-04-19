use hex_literal::hex;
use std::{borrow::Borrow, collections::BTreeMap, fs::File};

use ares_oracle_provider_support::crypto::sr25519::AuthorityId as AresId;
use log::log;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::{config::MultiaddrWithPeerId, ChainType};
use sc_telemetry::{serde_json, TelemetryEndpoints};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	app_crypto::sp_core::crypto::UncheckedFrom,
	traits::{IdentifyAccount, Verify},
	Perbill,
};

mod ares_genesis;
mod testnet_genesis;

use runtime_common::{AccountId, Balance};
use serde::Deserialize;
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};

const DEFAULT_PROTOCOL_ID: &str = "ares";

// use ares_genesis::make_ares_genesis;
pub use ares_genesis::{GladiosAccountId, GladiosNodeChainSpec, GladiosSS58Prefix, GladiosWASM_BINARY};
use testnet_genesis::{get_account_id_from_seed, make_testnet_genesis};
pub use testnet_genesis::{PioneerAccountId, PioneerNodeChainSpec, PioneerSS58Prefix, PioneerWASM_BINARY};

fn make_spec_config(config_path: Option<String>, default_config: &[u8], ss58: u16) -> Result<ChainSpecConfig, String> {
	let mut chain_spec_config: ChainSpecConfig;
	if let Some(path) = config_path {
		let path: &str = path.as_ref();
		let file = File::open(path).map_err(|e| format!("Error opening config file `{}`: {}", path, e))?;
		chain_spec_config = serde_yaml::from_reader(file).map_err(|e| format!("Error parsing config file: {}", e))?;
	} else {
		chain_spec_config =
			serde_yaml::from_slice(default_config).map_err(|e| format!("Error parsing config file: {}", e))?;
	}
	chain_spec_config.ss58 = Some(ss58);
	chain_spec_config.init();
	Ok(chain_spec_config)
}

pub fn make_pioneer_spec(config_path: Option<String>, default_config: &[u8]) -> Result<PioneerNodeChainSpec, String> {
	let chain_spec_config = make_spec_config(config_path, default_config, PioneerSS58Prefix::get().into())?;
	let name = chain_spec_config.name.clone();
	let id = chain_spec_config.id.clone();
	let chain_type = chain_spec_config.chain_type.clone();
	let boot_nodes = chain_spec_config.boot_nodes.clone();
	let telemetry_endpoints = chain_spec_config.telemetry_endpoints.clone();
	let wasm_binary = PioneerWASM_BINARY.ok_or_else(|| "Pioneer wasm not available".to_string())?;

	let mut properties = serde_json::map::Map::new();
	properties.insert("tokenDecimals".into(), (12 as u32).into());
	properties.insert("tokenSymbol".into(), "ARES".into());
	properties.insert("SS58Prefix".into(), PioneerSS58Prefix::get().into());

	// let chain_balance = &include_bytes!("./chain_spec/gladios-balance.json")[..];
	// let immigration: Vec<(AccountId, Balance)> = serde_json::from_slice(chain_balance).unwrap();
	Ok(PioneerNodeChainSpec::from_genesis(
		// Name
		name.as_ref(),
		// ID
		id.as_ref(),
		chain_type,
		move || make_testnet_genesis(wasm_binary, &chain_spec_config),
		boot_nodes.unwrap_or(vec![]),
		telemetry_endpoints,
		// Protocol ID
		None,
		// Properties
		None,
		Some(properties),
		// Extensions
		Default::default(),
	))
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChainSpecConfig {
	name: Box<str>,
	id: Box<str>,
	chain_type: ChainType,
	total_issuance: Balance,
	balances: Vec<(AccountId, Balance)>,
	collection: Vec<AccountId>,
	authorities: Vec<(
		AccountId, // stash
		AccountId, // controller
		BabeId,
		GrandpaId,
		AresId,
		ImOnlineId,
	)>,
	validator_minimum_deposit: Balance,
	council_minimum_deposit: Balance,
	root: AccountId,
	council: Vec<AccountId>,
	technical: Vec<AccountId>,
	boot_nodes: Option<Vec<MultiaddrWithPeerId>>,
	telemetry_endpoints: Option<TelemetryEndpoints>,
	symbols: Vec<(Box<str>, Box<str>, u32, u32, u8)>,
	ss58: Option<u16>,
}

impl ChainSpecConfig {
	pub fn init(&mut self) {
		let total_issuance: Balance = self.total_issuance.clone();
		let validator_minimum_deposit: Balance = self.validator_minimum_deposit.clone(); // Stake balance per validator.s
		let total_authorities_issuance: Balance =
			validator_minimum_deposit.saturating_mul((self.authorities.len() as u32).into());

		let mut minimum_balance = BTreeMap::<AccountId, Balance>::new();
		self.authorities.clone().iter().for_each(|(stash, controller, ..)| {
			minimum_balance.insert(stash.clone(), self.validator_minimum_deposit.clone());
		});
		self.council.clone().iter().for_each(|account| {
			if minimum_balance.contains_key(account) {
				minimum_balance.insert(
					account.clone(),
					minimum_balance.get(account).unwrap() + self.council_minimum_deposit.clone(),
				);
			} else {
				minimum_balance.insert(account.clone(), self.council_minimum_deposit.clone());
			}
		});

		let (mut account_balance_map, total_balances) = self.get_total_balance();
		minimum_balance.iter().for_each(|(accountId, minimum_amount)| {
			if account_balance_map.contains_key(accountId) {
				let bal = account_balance_map.get(accountId).unwrap();
				assert!(
					bal > minimum_amount,
					"account:{:?} balance is too low, pls reset. minimum:{}, current balance:{}",
					accountId.to_ss58check_with_version(Ss58AddressFormat::custom(self.ss58.unwrap())),
					minimum_amount,
					bal,
				);
			} else {
				assert!(
					false,
					"account:{:?} balance is empty, pls reset",
					accountId.to_ss58check_with_version(Ss58AddressFormat::custom(self.ss58.unwrap()))
				);
			}
		});
		println!("total_issuance:{}, total_balances:{}", total_issuance, total_balances);
		assert!(total_issuance > total_balances, "total_issuance can not greater than total_balances");
		let remaining = total_issuance - total_balances;
		if self.collection.len() > 0 {
			let collection_avg_balance = remaining.saturating_div((self.collection.len() as u32).into());
			for account in self.collection.iter() {
				if account_balance_map.contains_key(account) {
					let new_balance = account_balance_map.get(account).unwrap() + collection_avg_balance;
					account_balance_map.insert(account.clone(), new_balance);
					println!(
						"reset account:{}, balance:{}",
						account.to_ss58check_with_version(Ss58AddressFormat::custom(self.ss58.unwrap())),
						new_balance
					);
				} else {
					println!(
						"new account:{}, balance:{}",
						account.to_ss58check_with_version(Ss58AddressFormat::custom(self.ss58.unwrap())),
						collection_avg_balance
					);
					account_balance_map.insert(account.clone(), collection_avg_balance);
				}
			}
		} else {
			println!("spec_config.collection is empty, remaining balance:{}", remaining);
		}
		self.balances = account_balance_map.iter().map(|(a, b)| (a.clone(), b.clone())).collect();
	}

	fn get_total_balance(&self) -> (BTreeMap<AccountId, Balance>, Balance) {
		let mut unique_account = std::collections::BTreeMap::<AccountId, Balance>::new();
		let mut total_balance = Balance::default();
		self.balances.iter().for_each(|(account, amount)| {
			total_balance = total_balance.saturating_add(*amount);
			assert!(!unique_account.contains_key(account), "duplicate account {:?} ", account);
			unique_account.insert(account.clone(), *amount);
		});
		(unique_account, total_balance)
	}
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	// use crate::gladios_service::{new_full, new_light, new_partial};

	use sp_runtime::BuildStorage;

	#[test]
	fn test_staging_test_net_chain_spec() {
		let other_allocation_json = &include_bytes!("../res/ares.yml")[..];

		let other_allocation: ChainSpecConfig = serde_yaml::from_slice(other_allocation_json).unwrap();
		println!("{:?}", other_allocation);
		assert!(true);
	}
}
