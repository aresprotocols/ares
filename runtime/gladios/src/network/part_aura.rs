use super::*;
use constants::time::EPOCH_DURATION_IN_BLOCKS;
pub use pallet_aura;
use pallet_aura::Config;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = Session;
}
