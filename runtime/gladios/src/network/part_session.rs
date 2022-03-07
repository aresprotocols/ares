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
		pub babe: Babe,
		pub grandpa: Grandpa,
		pub ares: AresOracle,
		pub im_online: ImOnline,
	}
}

// TODO Remove while runtime upgrade is done.
// impl_opaque_keys! {
// 	pub struct OldSessionKeys {
// 		pub aura: Aura,
// 		pub grandpa: Grandpa,
// 		pub ares: AresOracle,
// 	}
// }

// pub fn dummy_imonline_id_from_account_id(a: AuraId) -> ImOnlineId {
// 	let mut id = ImOnlineId::default();
// 	let id_raw: &mut [u8] = id.as_mut();
//
// 	id_raw[0..].copy_from_slice(a.as_ref());
// 	id_raw[0..4].copy_from_slice(&sp_core::crypto::key_types::IM_ONLINE.0);
//
// 	id
// }

// When this is removed, should also remove `OldSessionKeys`.
// fn transform_session_keys(v: AccountId, old: OldSessionKeys) -> SessionKeys {
// 	SessionKeys {
// 		aura: old.aura.clone(),
// 		grandpa: old.grandpa,
// 		ares: old.ares,
// 		im_online: dummy_imonline_id_from_account_id(old.aura),
// 	}
// }

// When this is removed, should also remove `OldSessionKeys`.
// pub struct UpgradeSessionKeys;
// impl frame_support::traits::OnRuntimeUpgrade for UpgradeSessionKeys {
// 	fn on_runtime_upgrade() -> frame_support::weights::Weight {
// 		Session::upgrade_keys::<OldSessionKeys, _>(transform_session_keys);
// 		RuntimeBlockWeights::get().max_block / 2
// 	}
// }

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
