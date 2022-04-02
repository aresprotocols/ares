use super::*;

pub use runtime_gladios_node::{
    constants::currency::{Balance, CENTS},
    network::{
        part_elections::MAX_NOMINATIONS, part_session::SessionKeys, part_staking::StakerStatus,
        part_babe::BABE_GENESIS_EPOCH_CONFIG
    },
    AccountId, BabeConfig, BalancesConfig, CouncilConfig, DemocracyConfig, ElectionsConfig, ImOnlineConfig,
    GenesisConfig, GrandpaConfig, AresOracleConfig, SS58Prefix, SessionConfig, Signature,
    StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig, VestingConfig, WASM_BINARY as GladiosWASM_BINARY,

};
use sc_consensus_babe::AuthorityId as BabeId;
/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type GladiosNodeChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
pub type GladiosSS58Prefix = SS58Prefix;
pub type GladiosAccountId = AccountId;


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

fn session_keys(babe: BabeId, grandpa: GrandpaId, ares: AresId, im_online: ImOnlineId) -> SessionKeys {
    SessionKeys { babe, grandpa, ares, im_online }
    // SessionKeys { aura, grandpa, ares }
}

/// Configure initial storage state for FRAME modules.
pub fn make_ares_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, AresId, ImOnlineId)>,
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
            // changes_trie_config: Default::default(),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts.iter().cloned().map(|k| (k, endowment)).collect(),
        },
        // network
        // aura: AuraConfig {
        //     // authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        //     authorities: vec![],
        // },
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG),
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
                .map(|x| (x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone())))
                .collect::<Vec<_>>(),
        },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        ares_oracle: AresOracleConfig {
            _phantom: Default::default(),
            request_base: Vec::new(),
            price_pool_depth: 5u32,
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
