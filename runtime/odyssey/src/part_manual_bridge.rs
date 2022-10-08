use super::*;
use manual_bridge;

// parameter_types! {
// 	pub const MinimumBalanceThreshold: Balance = 1_000_000_000_000_000;
// }

impl manual_bridge::Config for Runtime {
    type Currency = pallet_balances::Pallet<Self>;
    type Event = Event;
    type RequestOrigin = frame_system::EnsureRoot<AccountId>;
    // type MinimumBalanceThreshold = MinimumBalanceThreshold;
    type WeightInfo = manual_bridge::weights::SubstrateWeight<Self>;
}