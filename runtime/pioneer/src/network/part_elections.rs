use super::*;
use constants::{currency::DOLLARS, currency::deposit, time::EPOCH_DURATION_IN_BLOCKS};
use governance::part_council::CouncilCollective;
pub use pallet_election_provider_multi_phase;
// use pallet_election_provider_multi_phase::FallbackStrategy;
use frame_election_provider_support::{ElectionDataProvider, onchain, SequentialPhragmen};
use frame_support::traits::{EitherOfDiverse};
use pallet_election_provider_multi_phase::SolutionAccuracyOf;
use part_babe::EpochDuration;
use runtime_common::prod_or_fast;
use sp_runtime::{transaction_validity::TransactionPriority, SaturatedConversion};
use sp_runtime::traits::{ConstU16,ConstU32};
//
// use super::*;
// use constants::{currency::DOLLARS, time::EPOCH_DURATION_IN_BLOCKS};
//
// use frame_support::traits::EnsureOneOf;
// use governance::part_council::CouncilCollective;
// pub use pallet_election_provider_multi_phase;
// use part_babe::EpochDuration;
// use runtime_common::prod_or_fast;
// use sp_runtime::{transaction_validity::TransactionPriority, SaturatedConversion};
//
// parameter_types! {
// 	// phase durations. 1/4 of the last session for each.
// 	// in testing: 1min or half of the session for each
// 	pub SignedPhase: u32 = prod_or_fast!(
// 		EPOCH_DURATION_IN_BLOCKS / 4,
// 		(1 * MINUTES).min(EpochDuration::get().saturated_into::<u32>() / 4),
// 		"ARES_SIGNED_PHASE"
// 	);
// 	pub UnsignedPhase: u32 = prod_or_fast!(
// 		EPOCH_DURATION_IN_BLOCKS / 4,
// 		(1 * MINUTES).min(EpochDuration::get().saturated_into::<u32>() / 4),
// 		"ARES_UNSIGNED_PHASE"
// 	);
//
// 	// signed config
// 	pub const SignedMaxSubmissions: u32 = 16;
// 	// 20 Ares fixed deposit..
// 	pub const SignedDepositBase: Balance = deposit(2, 0) * ARES_AMOUNT_MULT;
// 	// 0.005 Ares per KB of solution data.
// 	pub const SignedDepositByte: Balance = deposit(0, 10) / 1024 * ARES_AMOUNT_MULT;
// 	// Each good submission will get 1 DOT as reward
// 	pub SignedRewardBase: Balance = 1 * DOLLARS * ARES_AMOUNT_MULT;
// 	pub SolutionImprovementThreshold: Perbill = Perbill::from_rational(5u32, 10_000);
//
// 	// 2 hour session, 0.5 hour unsigned phase, 16 offchain executions.
// 	pub OffchainRepeat: BlockNumber = UnsignedPhase::get() / 32;
//
// 	/// Whilst `UseNominatorsAndUpdateBagsList` or `UseNominatorsMap` is in use, this can still be a
// 	/// very large value. Once the `BagsList` is in full motion, staking might open its door to many
// 	/// more nominators, and this value should instead be what is a "safe" number (e.g. 22500).
// 	pub const VoterSnapshotPerBlock: u32 = 22_500;
//
// 	// miner configs
// 	pub NposSolutionPriority: TransactionPriority = Perbill::from_percent(90) * TransactionPriority::max_value();
//
// }
//
// sp_npos_elections::generate_solution_type!(
// 	#[compact]
// 	pub struct NposCompactSolution16::<
// 		VoterIndex = u32,
// 		TargetIndex = u16,
// 		Accuracy = sp_runtime::PerU16,
// 	>(16)
// );
//
// /// on chain elect
// impl frame_election_provider_support::onchain::Config for Runtime {
// 	type Accuracy = Perbill;
// 	type DataProvider = staking_extend::data::DataProvider<Self>;
// }
//
// impl pallet_election_provider_multi_phase::Config for Runtime {
// 	type Event = Event;
// 	type Currency = Balances;
// 	type EstimateCallFee = TransactionPayment;
// 	type UnsignedPhase = UnsignedPhase;
// 	type SignedPhase = SignedPhase;
// 	type SolutionImprovementThreshold = SolutionImprovementThreshold;
// 	type OffchainRepeat = OffchainRepeat;
// 	type MinerTxPriority = NposSolutionPriority;
// 	type MinerMaxWeight = runtime_common::OffchainSolutionWeightLimit;
// 	type SignedMaxSubmissions = SignedMaxSubmissions;
// 	type SignedMaxWeight = Self::MinerMaxWeight;
// 	type SignedRewardBase = SignedRewardBase;
// 	type SignedDepositBase = SignedDepositBase;
// 	type SignedDepositByte = SignedDepositByte;
// 	type SignedDepositWeight = ();
// 	type VoterSnapshotPerBlock = VoterSnapshotPerBlock;
// 	type SlashHandler = ();
// 	// burn slashes
// 	type RewardHandler = ();
// 	// For now use the one from staking.
// 	type MinerMaxLength = runtime_common::OffchainSolutionLengthLimit;
// 	// nothing to do upon rewards
// 	type DataProvider = staking_extend::data::DataProvider<Self>;
// 	// problem
// 	type Solution = NposCompactSolution16;
// 	type Fallback = frame_election_provider_support::onchain::OnChainSequentialPhragmen<Self>; // pallet_election_provider_multi_phase::NoFallback<Self>;
// 	type GovernanceFallback = frame_election_provider_support::onchain::OnChainSequentialPhragmen<Self>;
// 	type Solver = frame_election_provider_support::SequentialPhragmen<
// 		AccountId,
// 		pallet_election_provider_multi_phase::SolutionAccuracyOf<Self>,
// 		runtime_common::OffchainRandomBalancing,
// 	>;
// 	type ForceOrigin = EnsureOneOf<
// 		EnsureRoot<AccountId>,
// 		pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>,
// 	>;
// 	type BenchmarkingConfig = runtime_common::BenchmarkConfig;
// 	type WeightInfo = pallet_election_provider_multi_phase::weights::SubstrateWeight<Runtime>;
// }

// ----------------------------------------

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
    type System = Runtime;
    type Solver = SequentialPhragmen<
        AccountId,
        pallet_election_provider_multi_phase::SolutionAccuracyOf<Runtime>,
    >;
    type DataProvider = <Runtime as pallet_election_provider_multi_phase::Config>::DataProvider;
    type WeightInfo = frame_election_provider_support::weights::SubstrateWeight<Runtime>;
}

impl onchain::BoundedConfig for OnChainSeqPhragmen {
    type VotersBound = MaxElectingVoters;
    type TargetsBound = ConstU32<2_000>;
}


parameter_types! {
	// phase durations. 1/4 of the last session for each.
	pub const SignedPhase: u32 = EPOCH_DURATION_IN_BLOCKS / 4;
	pub const UnsignedPhase: u32 = EPOCH_DURATION_IN_BLOCKS / 4;

	// signed config
	pub const SignedRewardBase: Balance = DOLLARS / 10 * ARES_AMOUNT_MULT;
	pub const SignedDepositBase: Balance = deposit(2, 0) * ARES_AMOUNT_MULT;
	pub const SignedDepositByte: Balance = deposit(0, 10) / 1024 * ARES_AMOUNT_MULT;

	pub BetterUnsignedThreshold: Perbill = Perbill::from_rational(1u32, 10_000);
	pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
	// miner configs
	pub const MultiPhaseUnsignedPriority: TransactionPriority = StakingUnsignedPriority::get() - 1u64;
	pub MinerMaxWeight: Weight = BlockWeights::get()
		.get(DispatchClass::Normal)
		.max_extrinsic.expect("Normal extrinsics have a weight limit configured; qed")
		.saturating_sub(BlockExecutionWeight::get());
	// Solution can occupy 90% of normal block size
	pub MinerMaxLength: u32 = Perbill::from_rational(9u32, 10) *
		*BlockLength::get()
		.max
		.get(DispatchClass::Normal);
	pub OffchainRepeat: BlockNumber = 5;
}

frame_election_provider_support::generate_solution_type!(
	#[compact]
	pub struct NposSolution16::<
		VoterIndex = u32,
		TargetIndex = u16,
		Accuracy = sp_runtime::PerU16,
		MaxVoters = MaxElectingVoters,
	>(16)
);

parameter_types! {
	pub MaxNominations: u32 = <NposSolution16 as frame_election_provider_support::NposSolution>::LIMIT as u32;
	pub MaxElectingVoters: u32 = 10_000;
}

type EnsureRootOrHalfCouncil = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;

impl pallet_election_provider_multi_phase::MinerConfig for Runtime {
    type AccountId = AccountId;
    type MaxLength = MinerMaxLength;
    type MaxWeight = MinerMaxWeight;
    type Solution = NposSolution16;
    type MaxVotesPerVoter =
    <<Self as pallet_election_provider_multi_phase::Config>::DataProvider as ElectionDataProvider>::MaxVotesPerVoter;

    // The unsigned submissions have to respect the weight of the submit_unsigned call, thus their
    // weight estimate function is wired to this call's weight.
    fn solution_weight(v: u32, t: u32, a: u32, d: u32) -> Weight {
        <
        <Self as pallet_election_provider_multi_phase::Config>::WeightInfo
        as
        pallet_election_provider_multi_phase::WeightInfo
        >::submit_unsigned(v, t, a, d)
    }
}

impl pallet_election_provider_multi_phase::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type EstimateCallFee = TransactionPayment;
    type SignedPhase = SignedPhase;
    type UnsignedPhase = UnsignedPhase;
    type BetterUnsignedThreshold = BetterUnsignedThreshold;
    type BetterSignedThreshold = ();
    type OffchainRepeat = OffchainRepeat;
    type MinerTxPriority = MultiPhaseUnsignedPriority;
    type MinerConfig = Self;
    type SignedMaxSubmissions = ConstU32<16>;
    type SignedRewardBase = SignedRewardBase;
    type SignedDepositBase = SignedDepositBase;
    type SignedDepositByte = SignedDepositByte;
    type SignedMaxRefunds = ConstU32<3>;
    type SignedDepositWeight = ();
    type SignedMaxWeight = MinerMaxWeight;
    type SlashHandler = (); // burn slashes
    type RewardHandler = (); // nothing to do upon rewards
    type DataProvider = staking_extend::data::DataProvider<Self>;
    type Fallback = onchain::BoundedExecution<OnChainSeqPhragmen>;
    type GovernanceFallback = onchain::BoundedExecution<OnChainSeqPhragmen>;
    type Solver = SequentialPhragmen<AccountId, SolutionAccuracyOf<Self>, OffchainRandomBalancing>;
    type ForceOrigin = EnsureRootOrHalfCouncil;
    type MaxElectableTargets = ConstU16<{ u16::MAX }>;
    type MaxElectingVoters = MaxElectingVoters;
    type BenchmarkingConfig = ElectionProviderBenchmarkConfig;
    type WeightInfo = pallet_election_provider_multi_phase::weights::SubstrateWeight<Self>;
}

pub struct ElectionProviderBenchmarkConfig;
impl pallet_election_provider_multi_phase::BenchmarkingConfig for ElectionProviderBenchmarkConfig {
    const VOTERS: [u32; 2] = [1000, 2000];
    const TARGETS: [u32; 2] = [500, 1000];
    const ACTIVE_VOTERS: [u32; 2] = [500, 800];
    const DESIRED_TARGETS: [u32; 2] = [200, 400];
    const SNAPSHOT_MAXIMUM_VOTERS: u32 = 1000;
    const MINER_MAXIMUM_VOTERS: u32 = 1000;
    const MAXIMUM_TARGETS: u32 = 300;
}