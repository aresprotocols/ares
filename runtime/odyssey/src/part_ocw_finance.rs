use frame_support::sp_std::marker::PhantomData;
use sp_runtime::traits::Convert;
use super::*;

pub type SessionIndex = u32;

parameter_types! {
	pub const AresFinancePalletId: PalletId = PalletId(*b"aoe/fund");
	pub const BasicDollars: Balance = DOLLARS;
	pub const AskPerEra: SessionIndex = 6;
	pub const HistoryDepth: u32 = 84;
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
	#[cfg(feature = "runtime-benchmarks")]
	type ValidatorIdOf = StashOfSelf<Self>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type WeightInfo = oracle_finance::weights::SubstrateWeight<Self>;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct StashOfSelf<T>(PhantomData<T>);

#[cfg(feature = "runtime-benchmarks")]
impl <T: oracle_finance::Config> Convert<T::AccountId, Option<T::AccountId>> for StashOfSelf<T> {
	fn convert(controller: T::AccountId) -> Option<T::AccountId> {
		Some(controller)
	}
}
