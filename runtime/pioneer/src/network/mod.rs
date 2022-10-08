use super::*;

parameter_types! {
	pub const MaxAuthorities: u32 = 100;
}

// pub mod part_aura;
pub mod part_authorship;
pub mod part_babe;
pub mod part_elections;
pub mod part_session;
pub mod part_staking;
pub mod part_staking_extend;
pub mod voter_bags;
pub mod part_nomination_pools;
pub mod part_authority_discovery;
// pub mod part_ranked_collective;
// pub mod part_referenda;
// pub mod part_conviction_voting;

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
