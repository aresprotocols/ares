use super::*;

pub type SessionIndex = u32;

parameter_types! {
	pub const AresFinancePalletId: PalletId = PalletId(*b"aoe/fund");
	pub const BasicDollars: Balance = DOLLARS;
	// pub const AskPeriod: BlockNumber = 1 * DAYS ;
	// pub const RewardPeriodCycle: AskPeriodNum = 100;
	// pub const RewardSlot: AskPeriodNum = 1; //
    pub AskPerEra: SessionIndex = 6;
    pub const HistoryDepth: u32 = 28;
}

impl oracle_finance::Config for Runtime {
    type Event = Event;
    type PalletId = AresFinancePalletId;
    type Currency = pallet_balances::Pallet<Self>;
    type BasicDollars = BasicDollars;
    // type AskPeriod = AskPeriod;
    // type RewardPeriodCycle = RewardPeriodCycle;
    // type RewardSlot = RewardSlot;
    type OnSlash = Treasury;
    type HistoryDepth = HistoryDepth;
    type SessionManager = ();
    type AskPerEra = AskPerEra;
    type ValidatorId = <Self as frame_system::Config>::AccountId;
}