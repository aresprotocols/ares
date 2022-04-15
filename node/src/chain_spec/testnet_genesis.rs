use super::*;

use runtime_common::{ Signature, AccountId};
pub use runtime_pioneer_node::{
	constants::currency::{Balance, CENTS},
	network::{part_babe::BABE_GENESIS_EPOCH_CONFIG, part_session::SessionKeys, part_staking::StakerStatus},
	AresOracleConfig, BabeConfig, BalancesConfig, Block, CouncilConfig, ClaimsConfig, DemocracyConfig, ElectionsConfig,
	GenesisConfig, GrandpaConfig, ImOnlineConfig, SS58Prefix, SessionConfig, StakingConfig, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, VestingConfig, WASM_BINARY as PioneerWASM_BINARY,
};

use serde::{Deserialize, Serialize};
use sc_chain_spec::ChainSpecExtension;

const TOTAL_ISSUANCE: Balance = 1_000_000_000 * DOLLARS; // one billion.
const PER_ELECTION_DESPOSIT: Balance = 2000 * DOLLARS;
const PER_STAKING_DESPOSIT: Balance = 2000 * DOLLARS; // Stake balance per validator.

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
use runtime_pioneer_node::DOLLARS;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type PioneerNodeChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
pub type PioneerSS58Prefix = SS58Prefix;
pub type PioneerAccountId = AccountId;
pub type PioneerBalance = Balance;

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
pub fn make_testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, AresId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	council_members: Vec<AccountId>,
	immigration_account: Option<Vec<(AccountId, Balance)>>,
	_enable_println: bool,
) -> GenesisConfig {

	let balance_accounts = merge_accounts_balance(
		// initial_authorities.len(),
		// council_members.len(),
		// endowed_accounts,
		// immigration_account,
		AresBalanceConfig{
			total_issuance: TOTAL_ISSUANCE,
			per_election_desposit: PER_ELECTION_DESPOSIT,
			per_staking_desposit: PER_STAKING_DESPOSIT,
			authorities_len: initial_authorities.len(),
			council_len: council_members.len(),
			endowed_accounts,
			immigration_account,
		}
	);

	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), PER_STAKING_DESPOSIT, StakerStatus::Validator))
		.collect::<Vec<_>>();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			// balances: endowed_accounts.iter().cloned().map(|k| (k, endowment)).collect(),
			balances: balance_accounts,
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
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
				.map(|x| (x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone())))
				.collect::<Vec<_>>(),
		},
		grandpa: GrandpaConfig { authorities: vec![] },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		ares_oracle: AresOracleConfig {
			_phantom: Default::default(),
			request_base: Vec::new(),
			price_pool_depth: 5u32,
			price_allowable_offset: 1u8,
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
			],
		},
		// council: CouncilConfig { phantom: Default::default(), members: council_members.clone() },
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig { phantom: Default::default(), members: council_members.clone() },
		claims: ClaimsConfig { claims: vec![], vesting: vec![] },
		vesting: VestingConfig { vesting: vec![] },
		treasury: Default::default(),
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: council_members
				.clone()
				.iter()
				.map(|member| (member.clone(), PER_ELECTION_DESPOSIT))
				.collect(),
		},
	}
}
