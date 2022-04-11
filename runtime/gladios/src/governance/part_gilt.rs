use frame_support::traits::EnsureOneOf;
use crate::governance::part_council::CouncilCollective;
use super::*;

parameter_types! {
	pub IgnoredIssuance: Balance = Treasury::pot();
	pub const QueueCount: u32 = 300;
	pub const MaxQueueLen: u32 = 1000;
	pub const FifoQueueLen: u32 = 250;
	pub const GiltPeriod: BlockNumber = 30 * DAYS;
	pub const MinFreeze: Balance = 10_000 * CENTS;
	pub const IntakePeriod: BlockNumber = 5 * MINUTES;
	pub const MaxIntakeBids: u32 = 100;
}

impl pallet_gilt::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type AdminOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>,
	>;
	type Deficit = (); // Mint
	type Surplus = (); // Burn
	type IgnoredIssuance = IgnoredIssuance;
	type QueueCount = QueueCount;
	type MaxQueueLen = MaxQueueLen;
	type FifoQueueLen = FifoQueueLen;
	type Period = GiltPeriod;
	type MinFreeze = MinFreeze;
	type IntakePeriod = IntakePeriod;
	type MaxIntakeBids = MaxIntakeBids;
	type WeightInfo = ();
}