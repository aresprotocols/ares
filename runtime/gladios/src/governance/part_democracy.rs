use super::*;
use frame_support::traits::EnsureOneOf;
use pallet_democracy;
use part_council::CouncilCollective;
use part_technical::TechnicalCollective;
use runtime_common::{prod_or_fast, *};

parameter_types! {
	pub LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "ARES_LAUNCH_PERIOD");
	pub VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "ARES_VOTING_PERIOD");
	pub FastTrackVotingPeriod: BlockNumber = prod_or_fast!(3 * HOURS, 1 * MINUTES, "ARES_FAST_TRACK_VOTING_PERIOD");
	pub const MinimumDeposit: Balance = 100 * CENTS * ARES_AMOUNT_MULT;
	pub EnactmentPeriod: BlockNumber = prod_or_fast!(8 * DAYS, 1, "ARES_ENACTMENT_PERIOD");
	pub CooloffPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "ARES_COOLOFF_PERIOD");
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = EnactmentPeriod; // Same as EnactmentPeriod
	type MinimumDeposit = MinimumDeposit;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin = pallet_ares_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>;
	/// A super-majority can have the next scheduled referendum be a straight majority-carries vote.
	// type ExternalMajorityOrigin =
	// 	pallet_ares_collective::EnsureProportionAtLeast<_3, sp_core::u32_trait::_5, AccountId,
	// CouncilCollective>;
	type ExternalMajorityOrigin = pallet_ares_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>;
	/// A unanimous council can have the next scheduled referendum be a straight default-carries
	/// (NTB) vote.
	type ExternalDefaultOrigin = pallet_ares_collective::EnsureProportionAtLeast<_1, _1, AccountId, CouncilCollective>;
	/// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin = pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, TechnicalCollective>;
	type InstantOrigin = pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, TechnicalCollective>;
	type InstantAllowed = InstantAllowed;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin = pallet_ares_collective::EnsureProportionAtLeast<_2, _3, AccountId, CouncilCollective>;
	// To cancel a proposal before it has been passed, the technical committee must be unanimous or
	// Root must agree.
	type CancelProposalOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, TechnicalCollective>,
	>;
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// Any single technical committee member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cool-off period.
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCollective>;
	type CooloffPeriod = CooloffPeriod;
	type PreimageByteDeposit = crate::governance::part_preimage::PreimageByteDeposit;
	type OperationalPreimageOrigin = pallet_ares_collective::EnsureMember<AccountId, CouncilCollective>;
	type Slash = Treasury;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
	type MaxVotes = MaxVotes;
	type WeightInfo = pallet_democracy::weights::SubstrateWeight<Runtime>;
	type MaxProposals = MaxProposals;
}
