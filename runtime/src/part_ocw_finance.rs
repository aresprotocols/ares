use super::*;

// pub type Balance = u64;
// pub type BlockNumber = u64;
pub type AskPeriodNum = u64;
// pub const DOLLARS: u64 = 1_000_000_000_000;

parameter_types! {
	pub const AresFinancePalletId: PalletId = PalletId(*b"ocw/fund");
	pub const BasicDollars: Balance = DOLLARS;
	pub const AskPeriod: BlockNumber = 100 ;
	pub const RewardPeriodCycle: AskPeriodNum = 20 * 24;
	pub const RewardSlot: AskPeriodNum = 1;
}

impl ocw_finance::Config for Runtime {
    type Event = Event;
    type PalletId = AresFinancePalletId;
    type Currency = pallet_balances::Pallet<Self>;
    type BasicDollars = BasicDollars;
    type AskPeriod = AskPeriod;
    type RewardPeriodCycle = RewardPeriodCycle;
    type RewardSlot = RewardSlot;
    type OnSlash = Treasury;
}