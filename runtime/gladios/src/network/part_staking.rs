use super::*;

use crate::network::part_elections::NposCompactSolution24;
use frame_support::traits::EnsureOneOf;
use frame_system::EnsureRoot;
use governance::part_council::CouncilCollective;
use pallet_collective;
use pallet_staking;
pub use pallet_staking::StakerStatus;
use sp_runtime::curve::PiecewiseLinear;
pub use sp_staking;

pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_750_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

// fn era_payout(
// 	total_staked: Balance,
// 	non_gilt_issuance: Balance,
// 	max_annual_inflation: Perquintill,
// 	period_fraction: Perquintill,
// 	auctioned_slots: u64,
// ) -> (Balance, Balance) {
// 	use pallet_staking_reward_fn::compute_inflation;
// 	use sp_arithmetic::traits::Saturating;
//
// 	let min_annual_inflation = Perquintill::from_rational(25u64, 1000u64);
// 	let delta_annual_inflation = max_annual_inflation.saturating_sub(min_annual_inflation);
//
// 	// 30% reserved for up to 60 slots.
// 	let auction_proportion = Perquintill::from_rational(auctioned_slots.min(60), 200u64);
//
// 	// Therefore the ideal amount at stake (as a percentage of total issuance) is 75% less the amount
// that we expect 	// to be taken up with auctions.
// 	let ideal_stake = Perquintill::from_percent(75).saturating_sub(auction_proportion);
//
// 	let stake = Perquintill::from_rational(total_staked, non_gilt_issuance);
// 	let falloff = Perquintill::from_percent(5);
// 	let adjustment = compute_inflation(stake, ideal_stake, falloff);
// 	let staking_inflation =
// 		min_annual_inflation.saturating_add(delta_annual_inflation * adjustment);
//
// 	let max_payout = period_fraction * max_annual_inflation * non_gilt_issuance;
// 	let staking_payout = (period_fraction * staking_inflation) * non_gilt_issuance;
// 	let rest = max_payout.saturating_sub(staking_payout);
//
// 	let other_issuance = non_gilt_issuance.saturating_sub(total_staked);
// 	if total_staked > other_issuance {
// 		let _cap_rest = Perquintill::from_rational(other_issuance, total_staked) * staking_payout;
// 		// We don't do anything with this, but if we wanted to, we could introduce a cap on the treasury
// amount 		// with: `rest = rest.min(cap_rest);`
// 	}
// 	(staking_payout, rest)
// }
//
// pub struct EraPayout;
// impl pallet_staking::EraPayout<Balance> for EraPayout {
// 	fn era_payout(
// 		total_staked: Balance,
// 		_total_issuance: Balance,
// 		era_duration_millis: u64,
// 	) -> (Balance, Balance) {
// 		// TODO: #3011 Update with proper auctioned slots tracking.
// 		// This should be fine for the first year of parachains.
// 		let auctioned_slots: u64 = auctions::Pallet::<Runtime>::auction_counter().into();
// 		const MAX_ANNUAL_INFLATION: Perquintill = Perquintill::from_percent(10);
// 		const MILLISECONDS_PER_YEAR: u64 = 1000 * 3600 * 24 * 36525 / 100;
//
// 		era_payout(
// 			total_staked,
// 			Gilt::issuance().non_gilt,
// 			MAX_ANNUAL_INFLATION,
// 			Perquintill::from_rational(era_duration_millis, MILLISECONDS_PER_YEAR),
// 			auctioned_slots,
// 		)
// 	}
// }

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
	// 24
	pub const MaxNominations: u32 = <NposCompactSolution24 as sp_npos_elections::NposSolution>::LIMIT as u32;
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
	type ElectionProvider = staking_extend::elect::OnChainSequentialPhragmen<Self>;
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
	type SlashCancelOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<_3, _4, AccountId, CouncilCollective>,
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
