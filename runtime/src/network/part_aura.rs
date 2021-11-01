use super::*;
use constants::time::EPOCH_DURATION_IN_BLOCKS;
pub use pallet_aura;
use pallet_aura::Config;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = Session;
}

// pub struct AuraSession<T>(sp_std::marker::PhantomData<(T)>);
//
// impl<T: Config> pallet_session::ShouldEndSession<T::BlockNumber> for AuraSession<T>
// where
// 	u64: From<<T as frame_system::Config>::BlockNumber>,
// {
// 	fn should_end_session(now: T::BlockNumber) -> bool {
// 		if now == 0 {
// 			return false;
// 		} else if now % EPOCH_DURATION_IN_BLOCKS == 0 {
// 			return true;
// 		}
// 		return false;
// 	}
// }
