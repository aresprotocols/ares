use super::*;

use frame_system::EnsureRootWithSuccess;
use sp_runtime::traits::ConstU16;

impl pallet_ranked_collective::Config for Runtime {
    type WeightInfo = pallet_ranked_collective::weights::SubstrateWeight<Self>;
    type Event = Event;
    type PromoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
    type DemoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
    type Polls = RankedPolls;
    type MinRankOfClass = traits::Identity;
    type VoteWeight = pallet_ranked_collective::Geometric;
}
