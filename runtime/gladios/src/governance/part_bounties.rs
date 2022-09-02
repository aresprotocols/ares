use super::*;
use pallet_bounties;
use part_treasury::{DataDepositPerByte, MaximumReasonLength};
use runtime_common::Balance;

parameter_types! {
	pub const BountyCuratorDeposit: Permill = Permill::from_percent(50);
	pub const BountyValueMinimum: Balance = 200 * CENTS * ARES_AMOUNT_MULT;
	pub const BountyDepositBase: Balance = 100 * CENTS * ARES_AMOUNT_MULT;
	pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
	pub const CuratorDepositMin: Balance = 1 * DOLLARS * ARES_AMOUNT_MULT;
	pub const CuratorDepositMax: Balance = 100 * DOLLARS * ARES_AMOUNT_MULT;
	pub const BountyDepositPayoutDelay: BlockNumber = 4 * DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 90 * DAYS;
}

// impl pallet_bounties::Config for Runtime {
// 	// type BountyDepositBase = BountyDepositBase;
// 	// type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
// 	// type BountyUpdatePeriod = BountyUpdatePeriod;
// 	type BountyCuratorDeposit = BountyCuratorDeposit;
// 	// type BountyValueMinimum = BountyValueMinimum;
// 	// type DataDepositPerByte = DataDepositPerByte;
// 	type ChildBountyManager = ();
// 	type Event = Event;
// 	type MaximumReasonLength = MaximumReasonLength;
// 	type WeightInfo = pallet_bounties::weights::SubstrateWeight<Runtime>;
// }

impl pallet_bounties::Config for Runtime {
	type Event = Event;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type BountyValueMinimum = BountyValueMinimum;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type WeightInfo = pallet_bounties::weights::SubstrateWeight<Runtime>;
	type ChildBountyManager = ChildBounties;
}
