use super::*;
use constants::time::EPOCH_DURATION_IN_BLOCKS;
pub use pallet_session;
use sp_runtime::{impl_opaque_keys, traits::OpaqueKeys};

parameter_types! {
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
	pub const Period: u32 = EPOCH_DURATION_IN_BLOCKS as u32; // 100 block = 10min  [10b = 1min] [10min = 100b]
	pub const Offset: u32 = 0;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
		pub grandpa: Grandpa,
		pub ares: AresOracle,
		pub im_online: ImOnline,
	}
}

impl pallet_session::Config for Runtime {
	type Event = Event;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = OracleFinance ; //pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}
