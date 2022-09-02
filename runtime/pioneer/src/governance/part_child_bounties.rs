use sp_runtime::traits::ConstU32;
use super::*;

parameter_types! {
	pub const ChildBountyValueMinimum: Balance = 1 * DOLLARS;
}

impl pallet_child_bounties::Config for Runtime {
    type Event = Event;
    type MaxActiveChildBountyCount = ConstU32<5>;
    type ChildBountyValueMinimum = ChildBountyValueMinimum;
    type WeightInfo = pallet_child_bounties::weights::SubstrateWeight<Runtime>;
}