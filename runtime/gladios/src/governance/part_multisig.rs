use super::*;
use runtime_common::*;
use constants::currency::deposit;
use pallet_multisig;

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = deposit(1, 88) * ARES_AMOUNT_MULT;
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = deposit(0, 32) * ARES_AMOUNT_MULT;
	pub const MaxSignatories: u16 = 100;
}

impl pallet_multisig::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}
