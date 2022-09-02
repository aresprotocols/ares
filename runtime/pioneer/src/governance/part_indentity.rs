use super::*;
use crate::governance::part_council::CouncilCollective;
use constants::currency::CENTS;
use frame_support::traits::{EitherOfDiverse};

type EnsureRootOrHalfCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;

parameter_types! {
	// Minimum 4 CENTS/byte
	pub const BasicDeposit: Balance = deposit(1, 258) * ARES_AMOUNT_MULT;
	pub const FieldDeposit: Balance = deposit(0, 66) * ARES_AMOUNT_MULT;
	pub const SubAccountDeposit: Balance = deposit(1, 53) * ARES_AMOUNT_MULT;
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type FieldDeposit = FieldDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = Treasury;
	type ForceOrigin = EnsureRootOrHalfCouncil;
	type RegistrarOrigin = EnsureRootOrHalfCouncil;
	type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>;
}
