use sc_consensus_babe::AuthorityId as BabeId;
use sp_runtime::Percent;

pub use odyssey_runtime::{
	governance::{part_elections::PhragmenElectionPalletId, part_treasury::TreasuryPalletId},
	network::{part_babe::BABE_GENESIS_EPOCH_CONFIG, part_session::SessionKeys, part_staking::StakerStatus},
	part_challenge::ChallengePalletId,
	part_estimates::EstimatesPalletId,
	part_ocw_finance::AresFinancePalletId,
	AresOracleConfig, BabeConfig, BalancesConfig, ClaimsConfig, CouncilConfig, DemocracyConfig, ElectionsConfig,
	GenesisConfig, GrandpaConfig, ImOnlineConfig, OracleFinanceConfig, SS58Prefix, SessionConfig, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, VestingConfig, WASM_BINARY,
};

use super::*;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, crate::chain_spec::Extensions>;

fn session_keys(babe: BabeId, grandpa: GrandpaId, ares: AresId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { babe, grandpa, ares, im_online }
}

pub fn make_spec(config_path: Option<String>, default_config: &[u8]) -> Result<ChainSpec, String> {
	let pallets = vec![
		EstimatesPalletId::get(),
		TreasuryPalletId::get(),
		PalletId(PhragmenElectionPalletId::get()),
		AresFinancePalletId::get(),
		ChallengePalletId::get(),
	];
	let chain_spec_config = make_spec_config(config_path, default_config, SS58Prefix::get().into(), pallets)?;
	let name = chain_spec_config.name.clone();
	let id = chain_spec_config.id.clone();
	let chain_type = chain_spec_config.chain_type.clone();
	let boot_nodes = chain_spec_config.boot_nodes.clone();
	let telemetry_endpoints = chain_spec_config.telemetry_endpoints.clone();
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Odyssey wasm not available".to_string())?;

	let mut properties = serde_json::map::Map::new();
	properties.insert("tokenDecimals".into(), (12 as u32).into());
	properties.insert("tokenSymbol".into(), "ARES".into());
	properties.insert("SS58Prefix".into(), SS58Prefix::get().into());

	// let chain_balance = &include_bytes!("./chain_spec/gladios-balance.json")[..];
	// let immigration: Vec<(AccountId, Balance)> = serde_json::from_slice(chain_balance).unwrap();
	Ok(ChainSpec::from_genesis(
		// Name
		name.as_ref(),
		// ID
		id.as_ref(),
		chain_type,
		move || make_genesis(wasm_binary, &chain_spec_config),
		boot_nodes.unwrap_or(vec![]),
		telemetry_endpoints,
		// Protocol ID
		Some("ares-odyssey"),
		// Properties
		None,
		Some(properties),
		// Extensions
		Default::default(),
	))
}

pub fn make_genesis(wasm_binary: &[u8], config: &ChainSpecConfig) -> GenesisConfig {
	// let mut rng = rand::thread_rng();
	let stakers = config
		.authorities
		.iter()
		.map(|(stash, controller, ..)| {
			(stash.clone(), controller.clone(), config.validator_minimum_deposit, StakerStatus::Validator)
		})
		.collect::<Vec<_>>();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		balances: BalancesConfig { balances: config.balances.clone() },
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		staking: StakingConfig {
			validator_count: config.authorities.len() as u32,
			minimum_validator_count: config.authorities.len() as u32,
			invulnerables: config.authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		session: SessionConfig {
			keys: config
				.authorities
				.iter()
				.map(|(stash, _controller, babe, grandpa, ares, im_online)| {
					(
						stash.clone(),
						stash.clone(),
						session_keys(babe.clone(), grandpa.clone(), ares.clone(), im_online.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		grandpa: GrandpaConfig { authorities: vec![] },
		oracle_finance: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(config.root.clone()),
		},
		ares_oracle: AresOracleConfig {
			_phantom: Default::default(),
			request_base: Vec::new(),
			price_pool_depth: 5u32,
			price_allowable_offset: Percent::from_percent(1),
			authorities: vec![],
			price_requests: config
				.symbols
				.iter()
				.map(|(price_key, request_uri, parse_version, fraction, interval)| {
					(
						price_key.as_bytes().to_vec(),
						request_uri.as_bytes().to_vec(),
						parse_version.clone(),
						fraction.clone(),
						interval.clone(),
					)
				})
				.collect(),
		},
		// council: CouncilConfig { phantom: Default::default(), members: council_members.clone() },
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			phantom: Default::default(),
			members: config.technical.clone(),
		},
		claims: ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: config
				.council
				.clone()
				.iter()
				.map(|member| (member.clone(), config.council_minimum_deposit))
				.collect(),
		},
	}
}
