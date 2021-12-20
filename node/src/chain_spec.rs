use hex_literal::hex;
use runtime_gladios_node::{
	constants::currency::{Balance, CENTS},
	network::{
		part_elections::MAX_NOMINATIONS, part_session::SessionKeys, part_staking::StakerStatus,
	},
	AccountId, AuraConfig, BalancesConfig, CouncilConfig, DemocracyConfig, ElectionsConfig,
	GenesisConfig, GrandpaConfig, AresOracleConfig, SS58Prefix, SessionConfig, Signature,
	StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig, VestingConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sc_telemetry::serde_json;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use ares_oracle_provider_support::crypto::sr25519::AuthorityId as AresId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public, H256};

use sp_runtime::{
	app_crypto::sp_core::crypto::UncheckedFrom,
	traits::{IdentifyAccount, Verify},
	Perbill,
};

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

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, AuraId, GrandpaId, AresId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<AresId>(seed),
	)
}

fn session_keys(aura: AuraId, grandpa: GrandpaId, ares: AresId) -> SessionKeys {
	SessionKeys { aura, grandpa, ares }
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties = serde_json::map::Map::new();
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("tokenSymbol".into(), "ARES".into());
	properties.insert("SS58Prefix".into(), SS58Prefix::get().into());
	// properties.insert("ss58Format".into(), SS58Prefix::get().into());

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
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
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let mut properties = serde_json::map::Map::new();
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("tokenSymbol".into(), "ARES".into());
	properties.insert("SS58Prefix".into(), SS58Prefix::get().into());

	let initial_authorities: Vec<(
		AccountId, // stash
		AccountId, // controller
		AuraId,
		GrandpaId,
		AresId,
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
		),
	];

	let endowed_accounts: Vec<AccountId> = vec![
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
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId, AresId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	council_members: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	const TOTAL_ISSUANCE: Balance = 10_0000_0000 * CENTS; // one billion
	let endowment: Balance = TOTAL_ISSUANCE / endowed_accounts.len() as u128;
	let elections_stash: Balance = endowment / 1000;

	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), elections_stash, StakerStatus::Validator))
		.collect::<Vec<_>>();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, endowment)).collect(),
		},
		// network
		aura: AuraConfig {
			// authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
			authorities: vec![],
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone())))
				.collect::<Vec<_>>(),
		},
		grandpa: GrandpaConfig {
			authorities: vec![],
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: root_key,
		},
		ares_oracle: AresOracleConfig {
			_phantom: Default::default(),
			request_base: Vec::new(),
			price_pool_depth: 3u32,
			price_allowable_offset: 10u8,
			authorities: vec![],
			price_requests: vec![
				// price_key, request_uri, parse_version, fraction_num, request interval
				("btc-usdt".as_bytes().to_vec(), "btc".as_bytes().to_vec(), 2u32, 4, 2),
				("eth-usdt".as_bytes().to_vec(), "eth".as_bytes().to_vec(), 2u32, 4, 2),
				("dot-usdt".as_bytes().to_vec(), "dot".as_bytes().to_vec(), 2u32, 4, 2),
				("link-usdt".as_bytes().to_vec(), "link".as_bytes().to_vec(), 2u32, 4, 2),

				("ada-usdt".as_bytes().to_vec(), "ada".as_bytes().to_vec(), 2u32, 4, 4),
				("xrp-usdt".as_bytes().to_vec(), "xrp".as_bytes().to_vec(), 2u32, 4, 4),
				("sol-usdt".as_bytes().to_vec(), "sol".as_bytes().to_vec(), 2u32, 4, 4),
				("uni-usdt".as_bytes().to_vec(), "uni".as_bytes().to_vec(), 2u32, 4, 4),
				("bnb-usdt".as_bytes().to_vec(), "bnb".as_bytes().to_vec(), 2u32, 4, 4),
				("1inch-usdt".as_bytes().to_vec(), "1inch".as_bytes().to_vec(), 2u32, 4, 4),
				("atom-usdt".as_bytes().to_vec(), "atom".as_bytes().to_vec(), 2u32, 4, 4),
				("trx-usdt".as_bytes().to_vec(), "trx".as_bytes().to_vec(), 2u32, 4, 4),
				("aave-usdt".as_bytes().to_vec(), "aave".as_bytes().to_vec(), 2u32, 4, 4),
				("snx-usdt".as_bytes().to_vec(), "snx".as_bytes().to_vec(), 2u32, 4, 4),

				("avax-usdt".as_bytes().to_vec(), "avax".as_bytes().to_vec(), 2u32, 4, 5),
				("ltc-usdt".as_bytes().to_vec(), "ltc".as_bytes().to_vec(), 2u32, 4, 5),
				("bch-usdt".as_bytes().to_vec(), "bch".as_bytes().to_vec(), 2u32, 4, 5),
				("fil-usdt".as_bytes().to_vec(), "fil".as_bytes().to_vec(), 2u32, 4, 5),
				("etc-usdt".as_bytes().to_vec(), "etc".as_bytes().to_vec(), 2u32, 4, 5),
				("eos-usdt".as_bytes().to_vec(), "eos".as_bytes().to_vec(), 2u32, 4, 5),
				("dash-usdt".as_bytes().to_vec(), "dash".as_bytes().to_vec(), 2u32, 4, 5),
				("comp-usdt".as_bytes().to_vec(), "comp".as_bytes().to_vec(), 2u32, 4, 5),
				("matic-usdt".as_bytes().to_vec(), "matic".as_bytes().to_vec(), 2u32, 4, 5),

				("doge-usdt".as_bytes().to_vec(), "doge".as_bytes().to_vec(), 2u32, 4, 8),
				("luna-usdt".as_bytes().to_vec(), "luna".as_bytes().to_vec(), 2u32, 4, 8),
				("ftt-usdt".as_bytes().to_vec(), "ftt".as_bytes().to_vec(), 2u32, 4, 8),
				("xlm-usdt".as_bytes().to_vec(), "xlm".as_bytes().to_vec(), 2u32, 4, 8),
				("vet-usdt".as_bytes().to_vec(), "vet".as_bytes().to_vec(), 2u32, 4, 8),
				("icp-usdt".as_bytes().to_vec(), "icp".as_bytes().to_vec(), 2u32, 4, 8),
				("theta-usdt".as_bytes().to_vec(), "theta".as_bytes().to_vec(), 2u32, 4, 8),
				("algo-usdt".as_bytes().to_vec(), "algo".as_bytes().to_vec(), 2u32, 4, 8),
				("xmr-usdt".as_bytes().to_vec(), "xmr".as_bytes().to_vec(), 2u32, 4, 8),
				("xtz-usdt".as_bytes().to_vec(), "xtz".as_bytes().to_vec(), 2u32, 4, 8),
				("egld-usdt".as_bytes().to_vec(), "egld".as_bytes().to_vec(), 2u32, 4, 8),
				("axs-usdt".as_bytes().to_vec(), "axs".as_bytes().to_vec(), 2u32, 4, 8),
				("iota-usdt".as_bytes().to_vec(), "iota".as_bytes().to_vec(), 2u32, 4, 8),
				("ftm-usdt".as_bytes().to_vec(), "ftm".as_bytes().to_vec(), 2u32, 4, 8),
				("ksm-usdt".as_bytes().to_vec(), "ksm".as_bytes().to_vec(), 2u32, 4, 4),
				("hbar-usdt".as_bytes().to_vec(), "hbar".as_bytes().to_vec(), 2u32, 4, 8),
				("neo-usdt".as_bytes().to_vec(), "neo".as_bytes().to_vec(), 2u32, 4, 8),
				("waves-usdt".as_bytes().to_vec(), "waves".as_bytes().to_vec(), 2u32, 4, 8),
				("mkr-usdt".as_bytes().to_vec(), "mkr".as_bytes().to_vec(), 2u32, 4, 8),
				("near-usdt".as_bytes().to_vec(), "near".as_bytes().to_vec(), 2u32, 4, 8),
				("btt-usdt".as_bytes().to_vec(), "btt".as_bytes().to_vec(), 2u32, 4, 8),
				("chz-usdt".as_bytes().to_vec(), "chz".as_bytes().to_vec(), 2u32, 4, 8),
				("stx-usdt".as_bytes().to_vec(), "stx".as_bytes().to_vec(), 2u32, 4, 8),
				("dcr-usdt".as_bytes().to_vec(), "dcr".as_bytes().to_vec(), 2u32, 4, 8),
				("xem-usdt".as_bytes().to_vec(), "xem".as_bytes().to_vec(), 2u32, 4, 8),
				("omg-usdt".as_bytes().to_vec(), "omg".as_bytes().to_vec(), 2u32, 4, 8),
				("zec-usdt".as_bytes().to_vec(), "zec".as_bytes().to_vec(), 2u32, 4, 8),
				("sushi-usdt".as_bytes().to_vec(), "sushi".as_bytes().to_vec(), 2u32, 4, 8),
				("enj-usdt".as_bytes().to_vec(), "enj".as_bytes().to_vec(), 2u32, 4, 8),
				("mana-usdt".as_bytes().to_vec(), "mana".as_bytes().to_vec(), 2u32, 4, 8),
				("yfi-usdt".as_bytes().to_vec(), "yfi".as_bytes().to_vec(), 2u32, 4, 8),
				("iost-usdt".as_bytes().to_vec(), "iost".as_bytes().to_vec(), 2u32, 4, 8),
				("qtum-usdt".as_bytes().to_vec(), "qtum".as_bytes().to_vec(), 2u32, 4, 8),
				("bat-usdt".as_bytes().to_vec(), "bat".as_bytes().to_vec(), 2u32, 4, 8),
				("zil-usdt".as_bytes().to_vec(), "zil".as_bytes().to_vec(), 2u32, 4, 8),
				("icx-usdt".as_bytes().to_vec(), "icx".as_bytes().to_vec(), 2u32, 4, 8),
				("grt-usdt".as_bytes().to_vec(), "grt".as_bytes().to_vec(), 2u32, 4, 8),
				("celo-usdt".as_bytes().to_vec(), "celo".as_bytes().to_vec(), 2u32, 4, 8),
				("zen-usdt".as_bytes().to_vec(), "zen".as_bytes().to_vec(), 2u32, 4, 8),
				("ren-usdt".as_bytes().to_vec(), "ren".as_bytes().to_vec(), 2u32, 4, 8),
				("sc-usdt".as_bytes().to_vec(), "sc".as_bytes().to_vec(), 2u32, 4, 8),
				("zrx-usdt".as_bytes().to_vec(), "zrx".as_bytes().to_vec(), 2u32, 4, 8),
				("ont-usdt".as_bytes().to_vec(), "ont".as_bytes().to_vec(), 2u32, 4, 8),
				("nano-usdt".as_bytes().to_vec(), "nano".as_bytes().to_vec(), 2u32, 4, 8),
				("crv-usdt".as_bytes().to_vec(), "crv".as_bytes().to_vec(), 2u32, 4, 8),
				("bnt-usdt".as_bytes().to_vec(), "bnt".as_bytes().to_vec(), 2u32, 4, 8),
				("fet-usdt".as_bytes().to_vec(), "fet".as_bytes().to_vec(), 2u32, 4, 8),
				("uma-usdt".as_bytes().to_vec(), "uma".as_bytes().to_vec(), 2u32, 4, 8),
				("iotx-usdt".as_bytes().to_vec(), "iotx".as_bytes().to_vec(), 2u32, 4, 8),
				("lrc-usdt".as_bytes().to_vec(), "lrc".as_bytes().to_vec(), 2u32, 4, 8),
				("sand-usdt".as_bytes().to_vec(), "sand".as_bytes().to_vec(), 2u32, 4, 8),
				("srm-usdt".as_bytes().to_vec(), "srm".as_bytes().to_vec(), 2u32, 4, 8),
				("kava-usdt".as_bytes().to_vec(), "kava".as_bytes().to_vec(), 2u32, 4, 8),
				("knc-usdt".as_bytes().to_vec(), "knc".as_bytes().to_vec(), 2u32, 4, 8),
			],
		},
		// council: CouncilConfig { phantom: Default::default(), members: council_members.clone() },
		council: CouncilConfig::default(),
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
				.map(|member| (member.clone(), elections_stash))
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
