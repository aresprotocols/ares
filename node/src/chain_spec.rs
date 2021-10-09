use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
	OCWModuleConfig,
	SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use hex_literal::hex;
use sp_runtime::app_crypto::sp_core::crypto::UncheckedFrom;

// use proc_macro::TokenStream;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()

}

pub fn gac(acc_raw: [u8; 32]) -> AccountId {
	AccountPublic::unchecked_from(acc_raw).into_account()
}

pub fn gau(aura_raw: [u8; 32], grand_raw: [u8; 32]) -> (AuraId, GrandpaId)  {
	// Public::from_slice(format_str);
	// let public_struct = TPublic::from_slice(&format_str);
	// public_struct.
	// TPublic::Pair::
	(AuraId::from_slice(&aura_raw), GrandpaId::from_slice(&grand_raw))
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			// testnet_genesis(
			// 	wasm_binary,
			// 	// Initial PoA authorities
			// 	vec![authority_keys_from_seed("Alice")],
			// 	// Sudo account
			// 	get_account_id_from_seed::<sr25519::Public>("Alice"),
			// 	// Pre-funded accounts
			// 	vec![
			// 		get_account_id_from_seed::<sr25519::Public>("Alice"),
			// 		get_account_id_from_seed::<sr25519::Public>("Bob"),
			// 		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			// 		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			// 	],
			// 	true,
			// )
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					// (gau(hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"]).into(),gau(hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"]).into()),
					// (gau(hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"]).into(),gau(hex!["74a173a22757ddc9790ed388953a1ed8a5933a421858533411b36ebd41d74165"]).into()),
					gau(hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"],hex!["3b7345bd36fb53c50be544a7c2847b9673984fa587af0c27108d3d464183e94f"]),
					gau(hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"],hex!["a16c71b78c13cbd73e09cc348be1e8521ec2ce4c2615d4f2cf0e8148ba454a05"]),
					gau(hex!["acad76a1f273ab3b8e453d630d347668f1cfa9b01605800dab7126a494c04c7c"],hex!["2ce72e098beb0bc8ed6c812099bed8c7c60ae8208c94abf4212d7fdeaf11bab3"]),
				],
				// Sudo account
				gac(hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"]),
				// Pre-funded accounts
				vec![
					gac(hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"]),
					gac(hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"]),
					gac(hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"]),
					gac(hex!["74a173a22757ddc9790ed388953a1ed8a5933a421858533411b36ebd41d74165"]),
					gac(hex!["acad76a1f273ab3b8e453d630d347668f1cfa9b01605800dab7126a494c04c7c"]),
					gac(hex!["9e55f821f7b3484f15942af308001c32f113f31444f420a77422702907510669"]),
					gac(hex!["4aa6e0eeed2e3d1f35a8eb1cd650451327ad378fb8975dbf5747016ff3be2460"]),
					gac(hex!["587bae319ecaee13ce2dbdedfc6d66eb189e5af427666b21b4d4a08c7af0671c"]),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		ocw_module: OCWModuleConfig {
			_phantom: Default::default(),
			request_base: Vec::new(),
			price_pool_depth: 3u32,
			price_allowable_offset: 10u8,
			price_requests: vec![
				// price_key, request_uri, parse_version, fraction_num, request interval
				("btc_price".as_bytes().to_vec(), "btcusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("eth_price".as_bytes().to_vec(), "ethusdt".as_bytes().to_vec(), 2u32, 4, 2),
				// ("eth_price,dot_price".as_bytes().to_vec(), "/api/getPartyPrice/ethusdt,dotusdt".as_bytes().to_vec(), 2u32, 4),
			]
		},
	}
}


#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, new_light_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	#[test]
	fn test_staging_test_net_chain_spec() {
		assert!(true);
	}
}