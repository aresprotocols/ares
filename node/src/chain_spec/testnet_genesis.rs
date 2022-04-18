use super::*;

use runtime_common::{AccountId, Signature};
pub use runtime_pioneer_node::{
	constants::currency::{Balance, CENTS},
	network::{part_babe::BABE_GENESIS_EPOCH_CONFIG, part_session::SessionKeys, part_staking::StakerStatus},
	AresOracleConfig, BabeConfig, BalancesConfig, Block, ClaimsConfig, CouncilConfig, DemocracyConfig, ElectionsConfig,
	GenesisConfig, GrandpaConfig, ImOnlineConfig, SS58Prefix, SessionConfig, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, VestingConfig, WASM_BINARY as PioneerWASM_BINARY,
};

use sc_chain_spec::ChainSpecExtension;
use serde::{Deserialize, Serialize};

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

use sc_consensus_babe::AuthorityId as BabeId;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type PioneerNodeChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
pub type PioneerSS58Prefix = SS58Prefix;
pub type PioneerAccountId = AccountId;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> PioneerAccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, BabeId, GrandpaId, AresId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<AresId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

fn session_keys(babe: BabeId, grandpa: GrandpaId, ares: AresId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { babe, grandpa, ares, im_online }
	// SessionKeys { aura, grandpa, ares }
}

/// Configure initial storage state for FRAME modules.
pub fn make_testnet_genesis(wasm_binary: &[u8], config: &ChainSpecConfig) -> GenesisConfig {
	let mut rng = rand::thread_rng();
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
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(config.root.clone()),
		},
		ares_oracle: AresOracleConfig {
			_phantom: Default::default(),
			request_base: Vec::new(),
			price_pool_depth: 5u32,
			price_allowable_offset: 1u8,
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
