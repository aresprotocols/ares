use std::{collections::BTreeMap, fs::File};

use ares_oracle_provider_support::crypto::sr25519::AuthorityId as AresId;
use frame_support::PalletId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{config::MultiaddrWithPeerId, ChainType};
use sc_telemetry::{serde_json, TelemetryEndpoints};
use serde::{Deserialize, Serialize};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{
	crypto::{Ss58AddressFormat, Ss58Codec},
	sr25519, Pair, Public,
};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{AccountIdConversion, IdentifyAccount, Verify},
	Perbill,
};

use runtime_common::{AccountId, Balance, Signature};

use crate::service::Block;

pub mod gladios;
pub mod pioneer;

#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, BabeId, GrandpaId, AresId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<AresId>(seed),
	)
}

fn make_spec_config(
	config_path: Option<String>,
	default_config: &[u8],
	ss58: u16,
	pallet_accounts: Vec<PalletId>,
) -> Result<ChainSpecConfig, String> {
	let mut chain_spec_config: ChainSpecConfig;
	if let Some(path) = config_path {
		let path: &str = path.as_ref();
		let file = File::open(path).map_err(|e| format!("Error opening config file `{}`: {}", path, e))?;
		chain_spec_config = serde_yaml::from_reader(file).map_err(|e| format!("Error parsing config file: {}", e))?;
	} else {
		chain_spec_config =
			serde_yaml::from_slice(default_config).map_err(|e| format!("Error parsing config file: {}", e))?;
	}
	let pallet_accounts: Vec<AccountId> = pallet_accounts.iter().map(|pallet| pallet.into_account()).collect();
	chain_spec_config.ban = [chain_spec_config.ban, pallet_accounts].concat();
	chain_spec_config.ss58 = Some(ss58);
	chain_spec_config.init();
	Ok(chain_spec_config)
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChainSpecConfig {
	name: Box<str>,
	id: Box<str>,
	chain_type: ChainType,
	total_issuance: Balance,
	balances: Vec<(AccountId, Balance)>,
	ban: Vec<AccountId>,
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

		let mut minimum_balance = BTreeMap::<AccountId, Balance>::new();
		self.authorities.clone().iter().for_each(|(stash, _controller, ..)| {
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
		minimum_balance.iter().for_each(|(account_id, minimum_amount)| {
			if account_balance_map.contains_key(account_id) {
				let bal = account_balance_map.get(account_id).unwrap();
				assert!(
					bal > minimum_amount,
					"account:{:?} balance is too low, pls reset. minimum:{}, current balance:{}",
					account_id.to_ss58check_with_version(Ss58AddressFormat::custom(self.ss58.unwrap())),
					minimum_amount,
					bal,
				);
			} else {
				assert!(
					false,
					"account:{:?} balance is empty, pls reset",
					account_id.to_ss58check_with_version(Ss58AddressFormat::custom(self.ss58.unwrap()))
				);
			}
		});
		println!("total_issuance:{}, total_balances:{}", total_issuance, total_balances);
		assert!(total_issuance > total_balances, "total_issuance can not greater than total_balances");
		let remaining = total_issuance - total_balances;
		if self.collection.len() > 0 {
			let collection_avg_balance = remaining.wrapping_div((self.collection.len() as u32).into());
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
			if self.ban.contains(account) {
				println!(
					"account:{} banned from list",
					account.to_ss58check_with_version(Ss58AddressFormat::custom(self.ss58.unwrap())),
				);
			} else {
				total_balance = total_balance.saturating_add(*amount);
				assert!(!unique_account.contains_key(account), "duplicate account {:?} ", account);
				unique_account.insert(account.clone(), *amount);
			}
		});
		(unique_account, total_balance)
	}
}

#[cfg(test)]
pub(crate) mod tests {
	use sp_runtime::BuildStorage;

	use super::*;

	// use crate::gladios_service::{new_full, new_light, new_partial};

	#[test]
	fn test_staging_test_net_chain_spec() {
		let other_allocation_json = &include_bytes!("../res/gladios.yml")[..];

		let other_allocation: ChainSpecConfig = serde_yaml::from_slice(other_allocation_json).unwrap();
		println!("{:?}", other_allocation);
		assert!(true);
	}
}
