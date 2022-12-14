use super::*;

use frame_support::sp_std::marker::PhantomData;
use sp_runtime::traits::Convert;
use frame_support::instances::{Instance1, Instance2};
use frame_support::traits::LockIdentifier;

pub type SessionIndex = u32;

parameter_types! {
	pub const AresFinancePalletId: PalletId = PalletId(*b"aoe/fund");
	pub const BasicDollars: Balance = DOLLARS;
	pub const AskPerEra: SessionIndex = 6;
	pub const HistoryDepth: u32 = 84;
	pub const OracleFinanceLockIdentifier : LockIdentifier = *b"aoe/fund";
	pub const ReminderFinanceLockIdentifier : LockIdentifier = *b"reminder";
}

impl oracle_finance::Config<Instance1> for Runtime {
	type Event = Event;
	type PalletId = AresFinancePalletId;
	type Currency = pallet_balances::Pallet<Self>;
	type BasicDollars = BasicDollars;
	type LockIdentifier = OracleFinanceLockIdentifier;
	type OnSlash = Treasury;
	type HistoryDepth = HistoryDepth;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type AskPerEra = AskPerEra;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	#[cfg(feature = "runtime-benchmarks")]
	type ValidatorIdOf = StashOfSelf<Self, Instance1>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type WeightInfo = oracle_finance::weights::SubstrateWeight<Self>;
}

impl oracle_finance::Config<Instance2> for Runtime {
	type Event = Event;
	type PalletId = AresFinancePalletId;
	type Currency = pallet_balances::Pallet<Self>;
	type BasicDollars = BasicDollars;
	type LockIdentifier = ReminderFinanceLockIdentifier;
	type OnSlash = Treasury;
	type HistoryDepth = HistoryDepth;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type AskPerEra = AskPerEra;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	#[cfg(feature = "runtime-benchmarks")]
	type ValidatorIdOf = StashOfSelf<Self, Instance2>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type WeightInfo = oracle_finance::weights::SubstrateWeight<Self>;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct StashOfSelf<T, I>(PhantomData<(T,I)>);

#[cfg(feature = "runtime-benchmarks")]
impl <T: oracle_finance::Config<I>, I: 'static> Convert<T::AccountId, Option<T::AccountId>> for StashOfSelf<T, I> {
	fn convert(controller: T::AccountId) -> Option<T::AccountId> {
		Some(controller)
	}
}