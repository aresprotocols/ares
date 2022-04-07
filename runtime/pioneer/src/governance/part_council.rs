use super::*;
pub use pallet_ares_collective;
use runtime_common::prod_or_fast;

parameter_types! {
	pub CouncilMotionDuration: BlockNumber = prod_or_fast!(1 * DAYS, 2 * MINUTES, "ARES_MOTION_DURATION");
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

pub type CouncilCollective = pallet_ares_collective::Instance1;
impl pallet_ares_collective::Config<CouncilCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_ares_collective::PrimeDefaultVote;
	type WeightInfo = pallet_ares_collective::weights::SubstrateWeight<Runtime>;
	type ChallengeFlow = AresChallenge;
	type AresProposalMinimumThreshold =
		pallet_ares_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>;
	type AresProposalMaximumThreshold =
		pallet_ares_collective::ares::EnsureProportionNoMoreThan<_2, _3, AccountId, CouncilCollective>;
}
