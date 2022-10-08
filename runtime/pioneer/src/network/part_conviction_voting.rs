use sp_runtime::traits::ConstU32;
use crate::*;

impl pallet_conviction_voting::Config for Runtime {
    type WeightInfo = pallet_conviction_voting::weights::SubstrateWeight<Self>;
    type Event = Event;
    type Currency = Balances;
    type VoteLockingPeriod = VoteLockingPeriod;
    type MaxVotes = ConstU32<512>;
    type MaxTurnout = frame_support::traits::TotalIssuanceOf<Balances, Self::AccountId>;
    type Polls = Referenda;
}