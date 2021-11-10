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
/// expand pallet_staking_reward_curve
/// const REWARD_CURVE: PiecewiseLinear<'static> = {
/// 	extern crate sp_runtime as _sp_runtime;
/// 	_sp_runtime::curve::PiecewiseLinear::<'static> {
/// 		points: &[
/// 			(
/// 				_sp_runtime::Perbill::from_parts(0u32),
/// 				_sp_runtime::Perbill::from_parts(2500_0000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(50000_0000u32),
/// 				_sp_runtime::Perbill::from_parts(1_0000_0000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(5_1474_3000u32),
/// 				_sp_runtime::Perbill::from_parts(86136000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(529486000u32),
/// 				_sp_runtime::Perbill::from_parts(74835000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(544229000u32),
/// 				_sp_runtime::Perbill::from_parts(65623000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(558972000u32),
/// 				_sp_runtime::Perbill::from_parts(58114000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(573715000u32),
/// 				_sp_runtime::Perbill::from_parts(51993000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(588456000u32),
/// 				_sp_runtime::Perbill::from_parts(47004000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(603197000u32),
/// 				_sp_runtime::Perbill::from_parts(42937000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(617937000u32),
/// 				_sp_runtime::Perbill::from_parts(39622000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(632675000u32),
/// 				_sp_runtime::Perbill::from_parts(36920000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(647415000u32),
/// 				_sp_runtime::Perbill::from_parts(34717000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(662156000u32),
/// 				_sp_runtime::Perbill::from_parts(32921000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(676897000u32),
/// 				_sp_runtime::Perbill::from_parts(31457000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(691632000u32),
/// 				_sp_runtime::Perbill::from_parts(30264000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(706375000u32),
/// 				_sp_runtime::Perbill::from_parts(29291000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(721114000u32),
/// 				_sp_runtime::Perbill::from_parts(28498000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(735842000u32),
/// 				_sp_runtime::Perbill::from_parts(27852000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(750579000u32),
/// 				_sp_runtime::Perbill::from_parts(27325000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(765292000u32),
/// 				_sp_runtime::Perbill::from_parts(26896000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(780013000u32),
/// 				_sp_runtime::Perbill::from_parts(26546000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(794712000u32),
/// 				_sp_runtime::Perbill::from_parts(26261000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(809448000u32),
/// 				_sp_runtime::Perbill::from_parts(26028000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(824189000u32),
/// 				_sp_runtime::Perbill::from_parts(25838000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(838837000u32),
/// 				_sp_runtime::Perbill::from_parts(25684000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(853524000u32),
/// 				_sp_runtime::Perbill::from_parts(25558000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(868243000u32),
/// 				_sp_runtime::Perbill::from_parts(25455000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(882966000u32),
/// 				_sp_runtime::Perbill::from_parts(25371000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(897571000u32),
/// 				_sp_runtime::Perbill::from_parts(25303000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(912311000u32),
/// 				_sp_runtime::Perbill::from_parts(25247000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(926819000u32),
/// 				_sp_runtime::Perbill::from_parts(25202000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(941413000u32),
/// 				_sp_runtime::Perbill::from_parts(25165000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(955889000u32),
/// 				_sp_runtime::Perbill::from_parts(25135000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(970009000u32),
/// 				_sp_runtime::Perbill::from_parts(25111000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(984340000u32),
/// 				_sp_runtime::Perbill::from_parts(25091000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(998289000u32),
/// 				_sp_runtime::Perbill::from_parts(25075000u32),
/// 			),
/// 			(
/// 				_sp_runtime::Perbill::from_parts(1000000000u32),
/// 				_sp_runtime::Perbill::from_parts(25073000u32),
/// 			),
/// 		],
/// 		maximum: _sp_runtime::Perbill::from_parts(100000000u32),
/// 	}
/// };

parameter_types! {
	pub const SessionsPerEra: sp_staking::SessionIndex = 1; // constants::time::EPOCH_DURATION_IN_BLOCKS (one session 10 min)
	pub const BondingDuration: pallet_staking::EraIndex = 1;
	pub const SlashDeferDuration: pallet_staking::EraIndex = 10; // 1/4 the bonding duration.
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
	type ElectionProvider =  ElectionProviderMultiPhase;
	// type ElectionProvider = StakingExtend;// // ElectionProviderMultiPhase;
	type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
		pallet_election_provider_multi_phase::OnChainConfig<Self>,
	>;
	// type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
	// 	staking_extend::OnChainConfig<Self>,
	// >;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}
