use super::*;

// use crate::network::part_elections::NposCompactSolution24;
use frame_support::traits::{EitherOfDiverse, U128CurrencyToVote};
use frame_system::EnsureRoot;
use governance::part_council::CouncilCollective;
use pallet_collective;
use pallet_staking;
pub use pallet_staking::StakerStatus;
use sp_runtime::curve::PiecewiseLinear;
use sp_runtime::traits::ConstU32;
pub use sp_staking;
use part_elections::MaxNominations;
use sp_npos_elections::VoteWeight;
use pallet_staking::UseNominatorsAndValidatorsMap;

pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_022_510,
		max_inflation: 0_080_000,
		ideal_stake: 0_750_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	// Six sessions in an era (3 hours).
	pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	// 14 eras for unbonding (42 hours).
	pub const BondingDuration: sp_staking::EraIndex = 84; // 28;
	// 13 eras in which slashes can be cancelled (slightly less than 39 hours).
	pub const SlashDeferDuration: sp_staking::EraIndex = 27;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominatorRewardedPerValidator: u32 = 256;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
	//
	// pub const MaxNominations: u32 = <NposSolution16 as frame_election_provider_support::NposSolution>::LIMIT as u32;
}

parameter_types! {
	pub const BagThresholds: &'static [u64] = &voter_bags::THRESHOLDS;
}

// impl pallet_bags_list::Config for Runtime {
// 	type Event = Event;
// 	type VoteWeightProvider = Staking;
// 	type WeightInfo = pallet_bags_list::weights::SubstrateWeight<Runtime>;
// 	type BagThresholds = BagThresholds;
// }

// parameter_types! {
// 	pub const BagThresholds: &'static [u64] = &voter_bags::THRESHOLDS;
// }

impl pallet_bags_list::Config for Runtime {
	type Event = Event;
	type ScoreProvider = Staking;
	type WeightInfo = pallet_bags_list::weights::SubstrateWeight<Runtime>;
	type BagThresholds = BagThresholds;
	type Score = VoteWeight;
}

impl pallet_staking::Config for Runtime {
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type UnixTime = Timestamp;
	// type CurrencyToVote = runtime_common::CurrencyToVote;
	type CurrencyToVote = U128CurrencyToVote;
	type ElectionProvider = staking_extend::elect::OnChainSequentialPhragmen<Self>;
	// TODO:: please check why not use : type GenesisElectionProvider = onchain::UnboundedExecution<OnChainSeqPhragmen>;
	type GenesisElectionProvider = staking_extend::elect::OnChainSequentialPhragmenGenesis<Self>;
	type MaxNominations = MaxNominations;
	type RewardRemainder = Treasury;
	type Event = Event;
	type Slash = Treasury;
	// send the slashed funds to the treasury.
	type Reward = ();
	// rewards are minted from the void
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	/// A super-majority of the council can cancel the slash.
	// type SlashCancelOrigin = EnsureOneOf<
	// 	EnsureRoot<AccountId>,
	// 	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>,
	// >;
	type SlashCancelOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>,
	>;
	type SessionInterface = Self;
	type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	// Alternatively, use pallet_staking::UseNominatorsMap<Runtime> to just use the nominators map.
	// Note that the aforementioned does not scale to a very large number of nominators.
	// type SortedListProvider = BagsList;
	type VoterList = UseNominatorsAndValidatorsMap<Self>;
	type MaxUnlockingChunks = ConstU32<32>;
	type OnStakerSlash = NominationPools;
	type BenchmarkingConfig = runtime_common::StakingBenchmarkingConfig;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}
