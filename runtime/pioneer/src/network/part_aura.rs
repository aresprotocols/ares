use super::*;
pub use pallet_aura;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

parameter_types! {
	pub const MaxAuthorities: u32 = 100;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = Session;
	type MaxAuthorities = MaxAuthorities;
}
