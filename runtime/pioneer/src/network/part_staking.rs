use super::*;

use frame_election_provider_support::onchain;
use frame_support::traits::{ConstU32, EnsureOneOf, U128CurrencyToVote};
use frame_system::{EnsureRoot};
use governance::part_council::CouncilCollective;
use pallet_ares_collective;
use pallet_staking;
pub use pallet_staking::StakerStatus;
use part_elections::MAX_NOMINATIONS;
use sp_runtime::curve::PiecewiseLinear;
pub use sp_staking::{self, EraIndex};

pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	// pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	pub const SessionsPerEra: sp_staking::SessionIndex = 1;
	pub const BondingDuration: EraIndex = 4;
	pub const SlashDeferDuration: EraIndex = 3; // 1/4 the bonding duration.
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominatorRewardedPerValidator: u32 = 256;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
	pub OffchainRepeat: BlockNumber = 5;
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

pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxNominators = ConstU32<1000>;
	type MaxValidators = ConstU32<1000>;
}

sp_npos_elections::generate_solution_type!(
	#[compact]
	pub struct NposSolution16::<
		VoterIndex = u32,
		TargetIndex = u16,
		Accuracy = sp_runtime::PerU16,
	>(16)
);

parameter_types! {
	pub MaxNominations: u32 = <NposSolution16 as sp_npos_elections::NposSolution>::LIMIT as u32;
}

impl pallet_staking::Config for Runtime {
	type MaxNominations = MaxNominations;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type CurrencyToVote = U128CurrencyToVote;
	type RewardRemainder = Treasury;
	type Event = Event;
	type Slash = Treasury; // send the slashed funds to the treasury.
	type Reward = (); // rewards are minted from the void
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
	// type ElectionProvider =  ElectionProviderMultiPhase;
	type ElectionProvider = staking_extend::elect::OnChainSequentialPhragmen<Self>; // // ElectionProviderMultiPhase;
									   // type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
									   // 	pallet_election_provider_multi_phase::OnChainConfig<Self>,
									   // >;
	type GenesisElectionProvider = staking_extend::elect::OnChainSequentialPhragmen<Self>;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
	// Alternatively, use pallet_staking::UseNominatorsMap<Runtime> to just use the nominators map.
	// Note that the aforementioned does not scale to a very large number of nominators.
	type SortedListProvider = BagsList;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
}
