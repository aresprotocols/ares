use super::*;
use pallet_price_estimates;

parameter_types! {
	pub const MinimumDeposit: Balance = 100 * DOLLARS * ARES_AMOUNT_MULT;
	pub const BidderMinimumDeposit: Balance = 1000 * DOLLARS * ARES_AMOUNT_MULT;
	pub const EstimatesPalletId: PalletId = PalletId(*b"py/arest");
	pub const EstimatesPerSymbol: u32 = 1;
	pub const UnsignedPriority: u64 = 1 << 20;
	pub const MaxQuotationDelay: BlockNumber = 600;
	pub const MaxEndDelay: BlockNumber = 60;
	pub const MaximumKeepLengthOfOldData: BlockNumber = 1 * DAYS;
}

impl pallet_price_estimates::Config for Runtime {
	type Event = Event;
	type PalletId = EstimatesPalletId;
	type MaxEstimatesPerSymbol = EstimatesPerSymbol;
	type Currency = Balances;
	type Call = Call;
	type PriceProvider = AresOracle;
	type OffchainAppCrypto = ares_oracle::ares_crypto::AresCrypto<AresId>;
	type UnsignedPriority = UnsignedPriority;
	type MaxQuotationDelay = MaxQuotationDelay;
	type MaxEndDelay = MaxEndDelay;
	type MaximumKeepLengthOfOldData = MaximumKeepLengthOfOldData;
	type WeightInfo = pallet_price_estimates::weights::SubstrateWeight<Self>;
}
