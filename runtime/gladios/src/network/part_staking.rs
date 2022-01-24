use super::*;

use frame_election_provider_support::onchain;
use frame_support::traits::U128CurrencyToVote;
use frame_system::{limits::BlockWeights, EnsureOneOf, EnsureRoot};
use governance::part_council::CouncilCollective;
use pallet_ares_collective;
use pallet_staking;
pub use pallet_staking::StakerStatus;
use part_elections::MAX_NOMINATIONS;
use sp_runtime::curve::PiecewiseLinear;
pub use sp_staking;
use staking_extend;


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
	pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	// pub const SessionsPerEra: sp_staking::SessionIndex = 1;
	pub const BondingDuration: pallet_staking::EraIndex = 24 * 28;
	pub const SlashDeferDuration: pallet_staking::EraIndex = 24 * 7; // 1/4 the bonding duration.
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominatorRewardedPerValidator: u32 = 256;
	pub OffchainRepeat: BlockNumber = 5;
}

impl pallet_staking::Config for Runtime {
	const MAX_NOMINATIONS: u32 = MAX_NOMINATIONS;
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
		AccountId,
		EnsureRoot<AccountId>,
		pallet_ares_collective::EnsureProportionAtLeast<_3, _4, AccountId, CouncilCollective>,
	>;
	type SessionInterface = Self;
	type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	// type ElectionProvider =  ElectionProviderMultiPhase;
	type ElectionProvider = StakingExtend;// // ElectionProviderMultiPhase;
	// type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
	// 	pallet_election_provider_multi_phase::OnChainConfig<Self>,
	// >;
	type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
		staking_extend::OnChainConfig<Self>,
	>;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}
