use super::*;
use pallet_price_estimates;

/// Configure the pallet-ares-demo in pallets/demo.
parameter_types! {
	pub const MinimumDeposit: Balance = 100 * DOLLARS;
	pub const BidderMinimumDeposit: Balance = 1000 * DOLLARS;
	pub const EstimatesPalletId: PalletId = PalletId(*b"py/arest");
	pub const EstimatesPerSymbol: u32 = 1;
	pub const UnsignedPriority: u64 = 1 << 20;
}

impl pallet_price_estimates::Config for Runtime {
	type Event = Event;
	type PalletId = EstimatesPalletId;
	type MaxEstimatesPerSymbol = EstimatesPerSymbol;
	type Currency = Balances;
	type Call = Call;
	type PriceProvider = AresOracle;
	type AuthorityId = ares_oracle::crypto2::AuraAuthId;
	type UnsignedPriority = UnsignedPriority;
}
