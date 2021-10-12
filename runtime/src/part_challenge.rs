use super::*;
use part_collective;
pub use pallet_ares_challenge;
pub use pallet_ares_collective;

/// Configure the pallet-ares-demo in pallets/demo.
parameter_types! {
	pub const MinimumDeposit: Balance = 100 * DOLLARS;
	pub const BidderMinimumDeposit: Balance = 1000 * DOLLARS;
	pub const DemoPalletId: PalletId = PalletId(*b"py/ardem");
}

impl pallet_ares_challenge::Config for Runtime {
    type Event = Event;
    type MinimumDeposit = MinimumDeposit;
    type PalletId = DemoPalletId;
    type CouncilMajorityOrigin = pallet_ares_collective::EnsureProportionAtLeast<_3, _4, AccountId, part_collective::CouncilCollective>;
    type Currency = Balances;
    type SlashProposer = AresChallenge;
    type BidderMinimumDeposit = BidderMinimumDeposit;
    type IsAuthority = Aura; //Aura Or Babe
    type AuthorityId = AuraId; // (Aura or Babe) AuthorityId
    // type FindAuthor = pallet_aura::FindAccountFromAuthorIndex<Self, Aura>;
}