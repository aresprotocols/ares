use super::*;
use frame_support::traits::EqualPrivilegeOnly;
use pallet_scheduler;

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		runtime_common::BlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
	// Retry a scheduled item every 10 blocks (1 minute) until the preimage exists.
	pub const NoPreimagePostponement: Option<u32> = Some(10);
}

// impl pallet_scheduler::Config for Runtime {
// 	type Event = Event;
// 	type Origin = Origin;
// 	type PalletsOrigin = OriginCaller;
// 	type Call = Call;
// 	type MaximumWeight = MaximumSchedulerWeight;
// 	type ScheduleOrigin = EnsureRoot<AccountId>;
// 	type OriginPrivilegeCmp = EqualPrivilegeOnly;
// 	type MaxScheduledPerBlock = MaxScheduledPerBlock;
// 	type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
// }

impl pallet_scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock; // type MaxScheduledPerBlock = ConstU32<50>;
	type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = NoPreimagePostponement;
}

// Used the compare the privilege of an origin inside the scheduler.
// pub struct OriginPrivilegeCmp;
// impl PrivilegeCmp<OriginCaller> for OriginPrivilegeCmp {
// 	fn cmp_privilege(left: &OriginCaller, right: &OriginCaller) -> Option<Ordering> {
// 		if left == right {
// 			return Some(Ordering::Equal)
// 		}
//
// 		match (left, right) {
// 			// Root is greater than anything.
// 			(OriginCaller::system(frame_system::RawOrigin::Root), _) => Some(Ordering::Greater),
// 			// Check which one has more yes votes.
// 			(
// 				OriginCaller::Council(pallet_ares_collective::RawOrigin::Members(l_yes_votes, l_count)),
// 				OriginCaller::Council(pallet_ares_collective::RawOrigin::Members(r_yes_votes, r_count)),
// 			) => Some((l_yes_votes * r_count).cmp(&(r_yes_votes * l_count))),
// 			// For every other origin we don't care, as they are not used for `ScheduleOrigin`.
// 			_ => None,
// 		}
// 	}
// }
