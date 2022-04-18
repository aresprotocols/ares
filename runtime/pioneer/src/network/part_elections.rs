use super::*;
use constants::{currency::DOLLARS, time::EPOCH_DURATION_IN_BLOCKS};

use frame_support::traits::EnsureOneOf;
use governance::part_council::CouncilCollective;
pub use pallet_election_provider_multi_phase;
use part_babe::EpochDuration;
use runtime_common::prod_or_fast;
use sp_runtime::{transaction_validity::TransactionPriority, SaturatedConversion};

parameter_types! {
	// phase durations. 1/4 of the last session for each.
	// in testing: 1min or half of the session for each
	pub SignedPhase: u32 = prod_or_fast!(
		EPOCH_DURATION_IN_BLOCKS / 4,
		(1 * MINUTES).min(EpochDuration::get().saturated_into::<u32>() / 4),
		"ARES_SIGNED_PHASE"
	);
	pub UnsignedPhase: u32 = prod_or_fast!(
		EPOCH_DURATION_IN_BLOCKS / 4,
		(1 * MINUTES).min(EpochDuration::get().saturated_into::<u32>() / 4),
		"ARES_UNSIGNED_PHASE"
	);

	// signed config
	pub const SignedMaxSubmissions: u32 = 16;
	// 20 Ares fixed deposit..
	pub const SignedDepositBase: Balance = deposit(2, 0) * ARES_AMOUNT_MULT;
	// 0.005 Ares per KB of solution data.
	pub const SignedDepositByte: Balance = deposit(0, 10) / 1024 * ARES_AMOUNT_MULT;
	// Each good submission will get 1 DOT as reward
	pub SignedRewardBase: Balance = 1 * DOLLARS * ARES_AMOUNT_MULT;
	pub SolutionImprovementThreshold: Perbill = Perbill::from_rational(5u32, 10_000);

	// 2 hour session, 0.5 hour unsigned phase, 16 offchain executions.
	pub OffchainRepeat: BlockNumber = UnsignedPhase::get() / 32;

	/// Whilst `UseNominatorsAndUpdateBagsList` or `UseNominatorsMap` is in use, this can still be a
	/// very large value. Once the `BagsList` is in full motion, staking might open its door to many
	/// more nominators, and this value should instead be what is a "safe" number (e.g. 22500).
	pub const VoterSnapshotPerBlock: u32 = 22_500;

	// miner configs
	pub NposSolutionPriority: TransactionPriority = Perbill::from_percent(90) * TransactionPriority::max_value();

}

sp_npos_elections::generate_solution_type!(
	#[compact]
	pub struct NposCompactSolution16::<
		VoterIndex = u32,
		TargetIndex = u16,
		Accuracy = sp_runtime::PerU16,
	>(16)
);

/// on chain elect
impl frame_election_provider_support::onchain::Config for Runtime {
	type Accuracy = Perbill;
	type DataProvider = staking_extend::data::DataProvider<Self>;
}

impl pallet_election_provider_multi_phase::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type EstimateCallFee = TransactionPayment;
	type UnsignedPhase = UnsignedPhase;
	type SignedPhase = SignedPhase;
	type SolutionImprovementThreshold = SolutionImprovementThreshold;
	type OffchainRepeat = OffchainRepeat;
	type MinerTxPriority = NposSolutionPriority;
	type MinerMaxWeight = runtime_common::OffchainSolutionWeightLimit;
	type SignedMaxSubmissions = SignedMaxSubmissions;
	type SignedMaxWeight = Self::MinerMaxWeight;
	type SignedRewardBase = SignedRewardBase;
	type SignedDepositBase = SignedDepositBase;
	type SignedDepositByte = SignedDepositByte;
	type SignedDepositWeight = ();
	type VoterSnapshotPerBlock = VoterSnapshotPerBlock;
	type SlashHandler = ();
	// burn slashes
	type RewardHandler = ();
	// For now use the one from staking.
	type MinerMaxLength = runtime_common::OffchainSolutionLengthLimit;
	// nothing to do upon rewards
	type DataProvider = staking_extend::data::DataProvider<Self>;
	// problem
	type Solution = NposCompactSolution16;
	type Fallback = pallet_election_provider_multi_phase::NoFallback<Self>;
	type GovernanceFallback = frame_election_provider_support::onchain::OnChainSequentialPhragmen<Self>;
	type Solver = frame_election_provider_support::SequentialPhragmen<
		AccountId,
		pallet_election_provider_multi_phase::SolutionAccuracyOf<Self>,
		runtime_common::OffchainRandomBalancing,
	>;
	type ForceOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_ares_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>,
	>;
	type BenchmarkingConfig = runtime_common::BenchmarkConfig;
	type WeightInfo = pallet_election_provider_multi_phase::weights::SubstrateWeight<Runtime>;
}
