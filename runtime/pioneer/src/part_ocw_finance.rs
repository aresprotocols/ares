use super::*;

pub type SessionIndex = u32;

parameter_types! {
	pub const AresFinancePalletId: PalletId = PalletId(*b"aoe/fund");
	pub const BasicDollars: Balance = DOLLARS;
	pub const AskPerEra: SessionIndex = 6;
	pub const HistoryDepth: u32 = 1000;
}

impl oracle_finance::Config for Runtime {
	type Event = Event;
	type PalletId = AresFinancePalletId;
	type Currency = pallet_balances::Pallet<Self>;
	type BasicDollars = BasicDollars;
	type OnSlash = Treasury;
	type HistoryDepth = HistoryDepth;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type AskPerEra = AskPerEra;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
}
