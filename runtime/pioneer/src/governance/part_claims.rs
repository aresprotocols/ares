
use super::*;
use polkadot_runtime_common::claims;
use crate::governance::part_council::CouncilCollective;

parameter_types! {
	pub Prefix: &'static [u8] = b"Pay ARES to the pioneer account:";
}

impl claims::Config for Runtime {
    type Event = Event;
    type VestingSchedule = Vesting;
    type Prefix = Prefix;
    type MoveClaimOrigin = pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>;
    type WeightInfo = EmptyClaimWeightInfo;
}

pub struct EmptyClaimWeightInfo;
impl polkadot_runtime_common::claims::WeightInfo for EmptyClaimWeightInfo {
    fn claim() -> Weight {
        0
    }
    fn mint_claim() -> Weight {
        0
    }
    fn claim_attest() -> Weight {
        0
    }
    fn attest() -> Weight {
        0
    }
    fn move_claim() -> Weight {
        0
    }
}