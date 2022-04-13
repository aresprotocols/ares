use super::*;
use constants::{
	currency::{DOLLARS, GRAND},
	time::DAYS,
};
use frame_support::traits::EnsureOneOf;
use frame_system::EnsureRoot;
use pallet_treasury;
use part_council::{self, CouncilCollective};
use sp_core::u32_trait::{_1, _2, _3, _5};
use sp_runtime::Percent;
use runtime_common::prod_or_fast;

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 2000 * CENTS * ARES_AMOUNT_MULT;
	pub const ProposalBondMaximum: Balance = 1 * GRAND * ARES_AMOUNT_MULT;
	// pub const SpendPeriod: BlockNumber = 6 * DAYS;
	pub SpendPeriod: BlockNumber = prod_or_fast!(
		6 * DAYS,
		6 * MINUTES,
		"ARES_UNSIGNED_PHASE"
	);
	pub const Burn: Permill = Permill::from_perthousand(2);
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaxApprovals: u32 = 100;
}

impl pallet_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_ares_collective::EnsureProportionAtLeast<_3, _5, AccountId, CouncilCollective>,
	>;
	type RejectOrigin = EnsureOneOf<
		EnsureRoot<AccountId>,
		pallet_ares_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>,
	>;
	type Event = Event;
	//type OnSlash = ();
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = Bounties;
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
	type MaxApprovals = MaxApprovals;
}
