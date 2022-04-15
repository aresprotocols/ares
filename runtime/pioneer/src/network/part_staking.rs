use super::*;

use crate::network::part_elections::NposCompactSolution16;
use frame_support::traits::EnsureOneOf;
use frame_system::EnsureRoot;
use governance::part_council::CouncilCollective;
use pallet_ares_collective;
use pallet_staking;
pub use pallet_staking::StakerStatus;
use runtime_common::prod_or_fast;
use sp_runtime::curve::PiecewiseLinear;
pub use sp_staking;

pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		// 3:2:1 staked : parachains : float.
		// while there's no parachains, then this is 75% staked : 25% float.
		ideal_stake: 0_750_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	// Six sessions in an era (12 hours).
	pub SessionsPerEra: sp_staking::SessionIndex = prod_or_fast!(6, 3, "ARES_SESSION_PER_ERA");
	// 28 eras for unbonding (14 days).
	pub const BondingDuration: sp_staking::EraIndex = 28;
	// 27 eras in which slashes can be cancelled (slightly less than 14 days).
	pub const SlashDeferDuration: sp_staking::EraIndex = 27;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominatorRewardedPerValidator: u32 = 256;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
	// 16
	pub const MaxNominations: u32 = <NposCompactSolution16 as sp_npos_elections::NposSolution>::LIMIT as u32;
}

parameter_types! {
	pub const BagThresholds: &'static [u64] = &voter_bags::THRESHOLDS;
}

impl pallet_bags_list::Config for Runtime {
	type Event = Event;
	type VoteWeightProvider = Staking;
	type WeightInfo = pallet_bags_list::weights::SubstrateWeight<Runtime>;
	type BagThresholds = BagThresholds;
}

impl pallet_staking::Config for Runtime {
	type Currency = Balances;
	type UnixTime = Timestamp;
	type CurrencyToVote = runtime_common::CurrencyToVote;
	// type ElectionProvider =  ElectionProviderMultiPhase;
	type ElectionProvider = staking_extend::elect::OnChainSequentialPhragmen<Self>;
	// // ElectionProviderMultiPhase;
	// type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
	// 	pallet_election_provider_multi_phase::OnChainConfig<Self>,
	// >;
	type GenesisElectionProvider = staking_extend::elect::OnChainSequentialPhragmen<Self>;
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
	type SlashCancelOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_ares_collective::EnsureProportionAtLeast<_3, _4, AccountId, CouncilCollective>,
	>;
	type SessionInterface = Self;
	type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	// Alternatively, use pallet_staking::UseNominatorsMap<Runtime> to just use the nominators map.
	// Note that the aforementioned does not scale to a very large number of nominators.
	type SortedListProvider = BagsList;
	type BenchmarkingConfig = runtime_common::StakingBenchmarkingConfig;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}
