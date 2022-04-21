use super::*;
use governance;
pub use pallet_ares_challenge;
pub use pallet_ares_collective;

parameter_types! {
	pub const MinimumDeposit: Balance = 100 * DOLLARS * ARES_AMOUNT_MULT;
	pub const BidderMinimumDeposit: Balance = 1000 * DOLLARS * ARES_AMOUNT_MULT;
	pub const ChallengePalletId: PalletId = PalletId(*b"py/ardem");
}

impl pallet_ares_challenge::Config for Runtime {
	type Event = Event;
	type MinimumDeposit = MinimumDeposit;
	type PalletId = ChallengePalletId;
	type CouncilMajorityOrigin =
		pallet_ares_collective::EnsureProportionAtLeast<_3, _4, AccountId, governance::part_council::CouncilCollective>;
	type Currency = Balances;
	type SlashProposer = AresChallenge;
	type BidderMinimumDeposit = BidderMinimumDeposit;
	type IsAuthority = Babe; //Aura Or Babe
	type AuthorityId = pallet_babe::AuthorityId; // (Aura or Babe) AuthorityId
											 // type FindAuthor = pallet_aura::FindAccountFromAuthorIndex<Self, Aura>;
}
