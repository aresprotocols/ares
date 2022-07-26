use super::*;
use pallet_price_estimates;

parameter_types! {
	pub const MinimumDeposit: Balance = 100 * DOLLARS * ARES_AMOUNT_MULT;
	pub const BidderMinimumDeposit: Balance = 1000 * DOLLARS * ARES_AMOUNT_MULT;
	pub const EstimatesPalletId: PalletId = PalletId(*b"py/arest");
	pub const EstimatesPerSymbol: u32 = 1;
	// pub const UnsignedPriority: u64 = 1 << 20;
	pub const MaxQuotationDelay: BlockNumber = 100;
}

impl pallet_price_estimates::Config for Runtime {
	type Event = Event;
	type PalletId = EstimatesPalletId;
	type MaxEstimatesPerSymbol = EstimatesPerSymbol;
	type Currency = Balances;
	type Call = Call;
	type PriceProvider = AresOracle;
	type AuthorityId = ares_oracle::ares_crypto::AresCrypto<AresId>;
	// type UnsignedPriority = UnsignedPriority;
	type MaxQuotationDelay = MaxQuotationDelay;
}
