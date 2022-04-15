
use super::*;
use polkadot_runtime_common::claims;
use polkadot_runtime_common::claims::TestWeightInfo;
use crate::governance::part_council::CouncilCollective;

parameter_types! {
	pub Prefix: &'static [u8] = b"Pay ARES to the pioneer account:";
}

impl claims::Config for Runtime {
    type Event = Event;
    type VestingSchedule = Vesting;
    type Prefix = Prefix;
    type MoveClaimOrigin =
    pallet_ares_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>;
    // type WeightInfo = polkadot_runtime_common::weights::runtime_common_claims::WeightInfo<Runtime>;
    type WeightInfo = TestWeightInfo; // No () impl
}
