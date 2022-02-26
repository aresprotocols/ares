use hex_literal::hex;

use sc_service::ChainType;
use sc_telemetry::serde_json;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use ares_oracle_provider_support::crypto::sr25519::AuthorityId as AresId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H256};

use sp_runtime::{
	app_crypto::sp_core::crypto::UncheckedFrom,
	traits::{IdentifyAccount, Verify},
	Perbill,
};

mod ares_genesis;
mod testnet_genesis;
const DEFAULT_PROTOCOL_ID: &str = "ares";

pub use ares_genesis::{GladiosNodeChainSpec, GladiosSS58Prefix, GladiosAccountId, GladiosWASM_BINARY};
pub use testnet_genesis::{PioneerNodeChainSpec, PioneerSS58Prefix, PioneerAccountId, PioneerWASM_BINARY};
use ares_genesis::make_ares_genesis;
use testnet_genesis::{make_testnet_genesis, get_account_id_from_seed};

// For dev config.
pub fn development_config() -> Result<PioneerNodeChainSpec, String> {
	let wasm_binary = GladiosWASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties = serde_json::map::Map::new();
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("tokenSymbol".into(), "ARES".into());
	properties.insert("SS58Prefix".into(), PioneerSS58Prefix::get().into());

	Ok(PioneerNodeChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			make_testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![testnet_genesis::authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				true,
			)
		},
		// Bootnodes
		vec!["/ip4/158.247.224.166/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp".parse().unwrap()],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

// For local testnet config
pub fn local_testnet_config() -> Result<PioneerNodeChainSpec, String> {
	let wasm_binary = PioneerWASM_BINARY.ok_or_else(|| "Local testnet wasm not available".to_string())?;
	let mut properties = serde_json::map::Map::new();
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("tokenSymbol".into(), "ARES".into());
	properties.insert("SS58Prefix".into(), PioneerSS58Prefix::get().into());

	let initial_authorities: Vec<(
		PioneerAccountId, // stash
		PioneerAccountId, // controller
		AuraId,
		GrandpaId,
		AresId,
		ImOnlineId,
	)> = vec![
		(
			hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"].into(),
			hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"].into(),
			hex!["08ecdc14e2dd427724c60c6879a1aeade21d9708c30c4477f679dde971cb1378"]
				.unchecked_into(),
			hex!["3b7345bd36fb53c50be544a7c2847b9673984fa587af0c27108d3d464183e94f"]
				.unchecked_into(),
			hex!["08ecdc14e2dd427724c60c6879a1aeade21d9708c30c4477f679dde971cb1378"]
				.unchecked_into(),
			hex!["08ecdc14e2dd427724c60c6879a1aeade21d9708c30c4477f679dde971cb1378"]
				.unchecked_into(),
		),
		(
			hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"].into(),
			hex!["74a173a22757ddc9790ed388953a1ed8a5933a421858533411b36ebd41d74165"].into(),
			hex!["46bd24b721b0252e4c5b933b3c1b53b5179799511594695bf03f06d17b91154e"]
				.unchecked_into(),
			hex!["a16c71b78c13cbd73e09cc348be1e8521ec2ce4c2615d4f2cf0e8148ba454a05"]
				.unchecked_into(),
			hex!["46bd24b721b0252e4c5b933b3c1b53b5179799511594695bf03f06d17b91154e"]
				.unchecked_into(),
			hex!["46bd24b721b0252e4c5b933b3c1b53b5179799511594695bf03f06d17b91154e"]
				.unchecked_into(),
		),
		(
			hex!["acad76a1f273ab3b8e453d630d347668f1cfa9b01605800dab7126a494c04c7c"].into(),
			hex!["9e55f821f7b3484f15942af308001c32f113f31444f420a77422702907510669"].into(),
			hex!["763a6ddd64b5e2f0e0c08a2c6e5143ae47edc563155bd052a26d3f942b806a1f"]
				.unchecked_into(),
			hex!["2ce72e098beb0bc8ed6c812099bed8c7c60ae8208c94abf4212d7fdeaf11bab3"]
				.unchecked_into(),
			hex!["763a6ddd64b5e2f0e0c08a2c6e5143ae47edc563155bd052a26d3f942b806a1f"]
				.unchecked_into(),
			hex!["763a6ddd64b5e2f0e0c08a2c6e5143ae47edc563155bd052a26d3f942b806a1f"]
				.unchecked_into(),
		),
		(
			hex!["4aa6e0eeed2e3d1f35a8eb1cd650451327ad378fb8975dbf5747016ff3be2460"].into(),
			hex!["587bae319ecaee13ce2dbdedfc6d66eb189e5af427666b21b4d4a08c7af0671c"].into(),
			hex!["a483a387dd54aa61d1619bfca66b41e0bbee9cd199306e4310f823526d6ebe6a"]
				.unchecked_into(),
			hex!["b200d0328d26f7cbb67223c179ab14a2152d7afb6689f07b618fda33695d5fd4"]
				.unchecked_into(),
			hex!["a483a387dd54aa61d1619bfca66b41e0bbee9cd199306e4310f823526d6ebe6a"]
				.unchecked_into(),
			hex!["a483a387dd54aa61d1619bfca66b41e0bbee9cd199306e4310f823526d6ebe6a"]
				.unchecked_into(),
		),
	];

	let endowed_accounts: Vec<PioneerAccountId> = vec![
		hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"].into(),
		hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"].into(),
		hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"].into(),
		hex!["74a173a22757ddc9790ed388953a1ed8a5933a421858533411b36ebd41d74165"].into(),
		hex!["acad76a1f273ab3b8e453d630d347668f1cfa9b01605800dab7126a494c04c7c"].into(),
		hex!["9e55f821f7b3484f15942af308001c32f113f31444f420a77422702907510669"].into(),
		hex!["4aa6e0eeed2e3d1f35a8eb1cd650451327ad378fb8975dbf5747016ff3be2460"].into(),
		hex!["587bae319ecaee13ce2dbdedfc6d66eb189e5af427666b21b4d4a08c7af0671c"].into(),
	];

	let council_members = endowed_accounts.clone();

	Ok(PioneerNodeChainSpec::from_genesis(
		// Name
		"Ares Pioneer",
		// ID
		"pioneer",
		ChainType::Live,
		move || {
			make_testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				initial_authorities.clone(),
				vec![],
				// Sudo account
				hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"].into(),
				// Pre-funded accounts
				endowed_accounts.clone(),
				council_members.clone(),
				true,
			)
		},
		// Bootnodes
		vec!["/ip4/158.247.224.166/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp".parse().unwrap()],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

//
pub fn local_ares_config() -> Result<GladiosNodeChainSpec, String> {
	let wasm_binary = GladiosWASM_BINARY.ok_or_else(|| "Gladios wasm not available".to_string())?;
	let mut properties = serde_json::map::Map::new();
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("tokenSymbol".into(), "ARES".into());
	properties.insert("SS58Prefix".into(), GladiosSS58Prefix::get().into());

	let initial_authorities: Vec<(
		GladiosAccountId, // stash
		GladiosAccountId, // controller
		AuraId,
		GrandpaId,
		AresId,
		ImOnlineId,
	)> = vec![
		(
			hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"].into(),
			hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"].into(),
			hex!["08ecdc14e2dd427724c60c6879a1aeade21d9708c30c4477f679dde971cb1378"]
				.unchecked_into(),
			hex!["3b7345bd36fb53c50be544a7c2847b9673984fa587af0c27108d3d464183e94f"]
				.unchecked_into(),
			hex!["08ecdc14e2dd427724c60c6879a1aeade21d9708c30c4477f679dde971cb1378"]
				.unchecked_into(),
			hex!["08ecdc14e2dd427724c60c6879a1aeade21d9708c30c4477f679dde971cb1378"]
				.unchecked_into(),
		),
		(
			hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"].into(),
			hex!["74a173a22757ddc9790ed388953a1ed8a5933a421858533411b36ebd41d74165"].into(),
			hex!["46bd24b721b0252e4c5b933b3c1b53b5179799511594695bf03f06d17b91154e"]
				.unchecked_into(),
			hex!["a16c71b78c13cbd73e09cc348be1e8521ec2ce4c2615d4f2cf0e8148ba454a05"]
				.unchecked_into(),
			hex!["46bd24b721b0252e4c5b933b3c1b53b5179799511594695bf03f06d17b91154e"]
				.unchecked_into(),
			hex!["46bd24b721b0252e4c5b933b3c1b53b5179799511594695bf03f06d17b91154e"]
				.unchecked_into(),
		),
		(
			hex!["acad76a1f273ab3b8e453d630d347668f1cfa9b01605800dab7126a494c04c7c"].into(),
			hex!["9e55f821f7b3484f15942af308001c32f113f31444f420a77422702907510669"].into(),
			hex!["763a6ddd64b5e2f0e0c08a2c6e5143ae47edc563155bd052a26d3f942b806a1f"]
				.unchecked_into(),
			hex!["2ce72e098beb0bc8ed6c812099bed8c7c60ae8208c94abf4212d7fdeaf11bab3"]
				.unchecked_into(),
			hex!["763a6ddd64b5e2f0e0c08a2c6e5143ae47edc563155bd052a26d3f942b806a1f"]
				.unchecked_into(),
			hex!["763a6ddd64b5e2f0e0c08a2c6e5143ae47edc563155bd052a26d3f942b806a1f"]
				.unchecked_into(),
		),
		(
			hex!["4aa6e0eeed2e3d1f35a8eb1cd650451327ad378fb8975dbf5747016ff3be2460"].into(),
			hex!["587bae319ecaee13ce2dbdedfc6d66eb189e5af427666b21b4d4a08c7af0671c"].into(),
			hex!["a483a387dd54aa61d1619bfca66b41e0bbee9cd199306e4310f823526d6ebe6a"]
				.unchecked_into(),
			hex!["b200d0328d26f7cbb67223c179ab14a2152d7afb6689f07b618fda33695d5fd4"]
				.unchecked_into(),
			hex!["a483a387dd54aa61d1619bfca66b41e0bbee9cd199306e4310f823526d6ebe6a"]
				.unchecked_into(),
			hex!["a483a387dd54aa61d1619bfca66b41e0bbee9cd199306e4310f823526d6ebe6a"]
				.unchecked_into(),
		),
	];

	let endowed_accounts: Vec<GladiosAccountId> = vec![
		hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"].into(),
		hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"].into(),
		hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"].into(),
		hex!["74a173a22757ddc9790ed388953a1ed8a5933a421858533411b36ebd41d74165"].into(),
		hex!["acad76a1f273ab3b8e453d630d347668f1cfa9b01605800dab7126a494c04c7c"].into(),
		hex!["9e55f821f7b3484f15942af308001c32f113f31444f420a77422702907510669"].into(),
		hex!["4aa6e0eeed2e3d1f35a8eb1cd650451327ad378fb8975dbf5747016ff3be2460"].into(),
		hex!["587bae319ecaee13ce2dbdedfc6d66eb189e5af427666b21b4d4a08c7af0671c"].into(),
	];

	let council_members = endowed_accounts.clone();

	Ok(GladiosNodeChainSpec::from_genesis(
		// Name
		"Ares Gladios",
		// ID
		"gladios",
		ChainType::Live,
		move || {
			make_ares_genesis(
				wasm_binary,
				// Initial PoA authorities
				initial_authorities.clone(),
				vec![],
				// Sudo account
				hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"].into(),
				// Pre-funded accounts
				endowed_accounts.clone(),
				council_members.clone(),
				true,
			)
		},
		// Bootnodes
		vec!["/ip4/45.77.243.246/tcp/30334/ws/p2p/12D3KooWMqDofvvwRtP7AYSAyJ2udNYrj698wnYJbpjomNTHHK9E".parse().unwrap()],
		// Telemetry
		None,
		// Protocol ID
		Some(DEFAULT_PROTOCOL_ID),
		// Properties
		Some(properties),
		// Extensions
		Default::default(),
	))
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	// use crate::gladios_service::{new_full, new_light, new_partial};
	use sp_runtime::BuildStorage;

	#[test]
	fn test_staging_test_net_chain_spec() {
		assert!(true);
	}
}
