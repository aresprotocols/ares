use super::*;
pub mod part_session;
pub mod part_staking;
pub mod part_elections;
pub mod part_aura;
pub mod part_authorship;


// ImOnline: pallet_im_online::{Pallet, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},

// impl pallet_im_online::Config for Runtime {
//     type AuthorityId = ImOnlineId;
//     type Event = Event;
//     type NextSessionRotation = Babe;
//     type ValidatorSet = Historical;
//     type ReportUnresponsiveness = Offences;
//     type UnsignedPriority = ImOnlineUnsignedPriority;
//     type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
// }