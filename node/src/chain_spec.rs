use hex_literal::hex;
use runtime_gladios_node::{
	constants::currency::{Balance, DOLLARS},
	AccountId, AuraConfig, BalancesConfig, CouncilConfig, DemocracyConfig, ElectionsConfig,
	GenesisConfig, GrandpaConfig, OCWModuleConfig, Signature, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, VestingConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::app_crypto::sp_core::crypto::UncheckedFrom;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sc_telemetry::serde_json;

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

pub fn gau(aura_raw: [u8; 32], grand_raw: [u8; 32]) -> (AuraId, GrandpaId) {
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
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		Some(
			serde_json::from_str(
				"{\"tokenDecimals\": 12, \"tokenSymbol\": \"ARES\", \"SS58Prefix\": 34}",
			).expect("Provided valid json map"),
		),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Ares Gladios",
		// ID
		"gladios",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					// (gau(hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"]).into(),gau(hex!["aaf0c45982a423036601dcacc67854b38b854690d8e15bf1543e9a00e660e019"]).into()),
					// (gau(hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"]).into(),gau(hex!["74a173a22757ddc9790ed388953a1ed8a5933a421858533411b36ebd41d74165"]).into()),
					gau(
						hex!["70214e02fb2ec155a4c7bb8c122864b3b03f58c4ac59e8d83af7dc29851df657"],
						hex!["3b7345bd36fb53c50be544a7c2847b9673984fa587af0c27108d3d464183e94f"],
					),
					gau(
						hex!["c82c3780d981812be804345618d27228680f61bb06a22689dcacf32b9be8815a"],
						hex!["a16c71b78c13cbd73e09cc348be1e8521ec2ce4c2615d4f2cf0e8148ba454a05"],
					),
					gau(
						hex!["acad76a1f273ab3b8e453d630d347668f1cfa9b01605800dab7126a494c04c7c"],
						hex!["2ce72e098beb0bc8ed6c812099bed8c7c60ae8208c94abf4212d7fdeaf11bab3"],
					),
					gau(
						hex!["4aa6e0eeed2e3d1f35a8eb1cd650451327ad378fb8975dbf5747016ff3be2460"],
						hex!["b200d0328d26f7cbb67223c179ab14a2152d7afb6689f07b618fda33695d5fd4"],
					),
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
	council_members: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const ELECTIONS_STASH: Balance = ENDOWMENT / 1000;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
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
				("btc-usdt".as_bytes().to_vec(), "btcusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("eth-usdt".as_bytes().to_vec(), "ethusdt".as_bytes().to_vec(), 2u32, 4, 2),
				// ("eth_price,dot_price".as_bytes().to_vec(), "/api/getPartyPrice/ethusdt,dotusdt".as_bytes().to_vec(), 2u32, 4),
				("ada-usdt".as_bytes().to_vec(), "adausdt".as_bytes().to_vec(), 2u32, 4, 2),
				("xrp-usdt".as_bytes().to_vec(), "xrpusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("sol-usdt".as_bytes().to_vec(), "solusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("dot-price".as_bytes().to_vec(), "dotusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("doge-usdt".as_bytes().to_vec(), "dogeusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("avax-usdt".as_bytes().to_vec(), "avaxusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("luna-usdt".as_bytes().to_vec(), "lunausdt".as_bytes().to_vec(), 2u32, 4, 2),
				("uni-usdt".as_bytes().to_vec(), "uniusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("ltc-usdt".as_bytes().to_vec(), "ltcusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("bch-usdt".as_bytes().to_vec(), "bchusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("link-usdt".as_bytes().to_vec(), "linkusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("ftt-usdt".as_bytes().to_vec(), "fttusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("atom-usdt".as_bytes().to_vec(), "atomusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("fil-usdt".as_bytes().to_vec(), "filusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("matic-usdt".as_bytes().to_vec(), "maticusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("trx-usdt".as_bytes().to_vec(), "trxusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("xlm-usdt".as_bytes().to_vec(), "xlmusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("vet-usdt".as_bytes().to_vec(), "vetusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("icp-usdt".as_bytes().to_vec(), "icpusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("etc-usdt".as_bytes().to_vec(), "etcusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("theta-usdt".as_bytes().to_vec(), "thetausdt".as_bytes().to_vec(), 2u32, 4, 2),
				("algo-usdt".as_bytes().to_vec(), "algousdt".as_bytes().to_vec(), 2u32, 4, 2),
				("xmr-usdt".as_bytes().to_vec(), "xmrusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("xtz-usdt".as_bytes().to_vec(), "xtzusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("eos-usdt".as_bytes().to_vec(), "eosusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("egld-usdt".as_bytes().to_vec(), "egldusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("axs-usdt".as_bytes().to_vec(), "axsusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("aave-usdt".as_bytes().to_vec(), "aaveusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("iota-usdt".as_bytes().to_vec(), "iotausdt".as_bytes().to_vec(), 2u32, 4, 2),
				("ftm-usdt".as_bytes().to_vec(), "ftmusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("ksm-usdt".as_bytes().to_vec(), "ksmusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("hbar-usdt".as_bytes().to_vec(), "hbarusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("neo-usdt".as_bytes().to_vec(), "neousdt".as_bytes().to_vec(), 2u32, 4, 2),
				("waves-usdt".as_bytes().to_vec(), "wavesusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("mkr-usdt".as_bytes().to_vec(), "mkrusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("near-usdt".as_bytes().to_vec(), "nearusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("btt-usdt".as_bytes().to_vec(), "bttusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("dash-usdt".as_bytes().to_vec(), "dashusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("snx-usdt".as_bytes().to_vec(), "snxusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("chz-usdt".as_bytes().to_vec(), "chzusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("comp-usdt".as_bytes().to_vec(), "compusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("stx-usdt".as_bytes().to_vec(), "stxusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("dcr-usdt".as_bytes().to_vec(), "dcrusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("xem-usdt".as_bytes().to_vec(), "xemusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("omg-usdt".as_bytes().to_vec(), "omgusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("zec-usdt".as_bytes().to_vec(), "zecusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("sushi-usdt".as_bytes().to_vec(), "sushiusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("enj-usdt".as_bytes().to_vec(), "enjusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("mana-usdt".as_bytes().to_vec(), "manausdt".as_bytes().to_vec(), 2u32, 4, 2),
				("yfi-usdt".as_bytes().to_vec(), "yfiusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("iost-usdt".as_bytes().to_vec(), "iostusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("qtum-usdt".as_bytes().to_vec(), "qtumusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("bat-usdt".as_bytes().to_vec(), "batusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("zil-usdt".as_bytes().to_vec(), "zilusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("icx-usdt".as_bytes().to_vec(), "icxusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("grt-usdt".as_bytes().to_vec(), "grtusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("celo-usdt".as_bytes().to_vec(), "celousdt".as_bytes().to_vec(), 2u32, 4, 2),
				("zen-usdt".as_bytes().to_vec(), "zenusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("ren-usdt".as_bytes().to_vec(), "renusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("sc-usdt".as_bytes().to_vec(), "scusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("zrx-usdt".as_bytes().to_vec(), "zrxusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("ont-usdt".as_bytes().to_vec(), "ontusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("nano-usdt".as_bytes().to_vec(), "nanousdt".as_bytes().to_vec(), 2u32, 4, 2),
				("crv-usdt".as_bytes().to_vec(), "crvusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("bnt-usdt".as_bytes().to_vec(), "bntusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("fet-usdt".as_bytes().to_vec(), "fetusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("uma-usdt".as_bytes().to_vec(), "umausdt".as_bytes().to_vec(), 2u32, 4, 2),
				("iotx-usdt".as_bytes().to_vec(), "iotxusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("lrc-usdt".as_bytes().to_vec(), "lrcusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("sand-usdt".as_bytes().to_vec(), "sandusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("srm-usdt".as_bytes().to_vec(), "srmusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("1INCH-usdt".as_bytes().to_vec(), "1INCHusdt".as_bytes().to_vec(), 2u32, 4, 2),
				("kava-usdt".as_bytes().to_vec(), "kavausdt".as_bytes().to_vec(), 2u32, 4, 2),
			],
		},
		// council: CouncilConfig { phantom: Default::default(), members: council_members.clone() },
		council: CouncilConfig::default(),
		// TODO fix members
		technical_committee: TechnicalCommitteeConfig {
			phantom: Default::default(),
			members: council_members.clone(),
		},
		vesting: VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: council_members
				.clone()
				.iter()
				.map(|member| (member.clone(), ELECTIONS_STASH))
				.collect(),
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
