use super::*;
use constants::currency::deposit;
use frame_support::traits::{LockIdentifier, U128CurrencyToVote};
use pallet_elections_phragmen;
use part_council::CouncilMaxMembers;
use runtime_common::prod_or_fast;
use static_assertions::const_assert;

parameter_types! {
	pub const CandidacyBond: Balance = 100 * CENTS * ARES_AMOUNT_MULT;
	// 1 storage item created, key size is 32 bytes, value size is 16+16.
	pub const VotingBondBase: Balance = deposit(1, 64) * ARES_AMOUNT_MULT;
	// additional data per vote is 32 bytes (account id).
	pub const VotingBondFactor: Balance = deposit(0, 32) * ARES_AMOUNT_MULT;
	/// Weekly council elections; scaling up to monthly eventually.
	pub TermDuration: BlockNumber = prod_or_fast!(24 * HOURS, 2 * MINUTES, "ARES_TERM_DURATION");
	/// 13 members initially, to be increased to 23 eventually.
	pub const DesiredMembers: u32 = 10;
	pub const DesiredRunnersUp: u32 = 10;
	pub const MaxVoters: u32 = 10 * 1000;
	pub const MaxCandidates: u32 = 1000;
	pub const PhragmenElectionPalletId: LockIdentifier = *b"phrelect";
}

// Make sure that there are no more than `MaxMembers` members elected via elections-phragmen.
const_assert!(DesiredMembers::get() <= CouncilMaxMembers::get());

impl pallet_elections_phragmen::Config for Runtime {
	type Event = Event;
	type PalletId = PhragmenElectionPalletId;
	type Currency = Balances;
	type ChangeMembers = Council;
	// NOTE: this implies that council's genesis members cannot be set directly and must come from
	// this module.
	type InitializeMembers = Council;
	type CurrencyToVote = U128CurrencyToVote;
	type CandidacyBond = CandidacyBond;
	type VotingBondBase = VotingBondBase;
	type VotingBondFactor = VotingBondFactor;

	type LoserCandidate = Treasury;
	type KickedMember = Treasury;

	type DesiredMembers = DesiredMembers;
	type DesiredRunnersUp = DesiredRunnersUp;
	type TermDuration = TermDuration;
	type MaxVoters = MaxVoters;
	type MaxCandidates = MaxCandidates;
	type WeightInfo = pallet_elections_phragmen::weights::SubstrateWeight<Runtime>;
}
