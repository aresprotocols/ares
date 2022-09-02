use super::*;
use governance;
pub use pallet_ares_challenge;

parameter_types! {
	pub const MinimumDeposit: Balance = 100 * DOLLARS * ARES_AMOUNT_MULT;
	pub const BidderMinimumDeposit: Balance = 1000 * DOLLARS * ARES_AMOUNT_MULT;
	pub const ChallengePalletId: PalletId = PalletId(*b"py/ardem");
	pub const MinimumThreshold: u32 = governance::part_elections::DesiredMembers::get() / 3 * 2;
}

pub type Challenge = pallet_ares_challenge::Instance1;
impl pallet_ares_challenge::Config<Challenge> for Runtime {
	type Event = Event;
	type MinimumDeposit = MinimumDeposit;
	type PalletId = ChallengePalletId;
	type CouncilMajorityOrigin = pallet_collective::EnsureProportionAtLeast<AccountId, governance::part_council::CouncilCollective,3,4>;
	type Currency = Balances;
	type SlashProposer = AresChallenge;
	// type BidderMinimumDeposit = BidderMinimumDeposit;
	type IsAuthority = Babe; //Aura Or Babe
	type AuthorityId = pallet_babe::AuthorityId;
	type Proposal = Call; // (Aura or Babe) AuthorityId
 	// type FindAuthor = pallet_aura::FindAccountFromAuthorIndex<Self, Aura>;
	type MinimumThreshold = MinimumThreshold;
}
