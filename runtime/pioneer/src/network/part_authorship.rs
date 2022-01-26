use super::*;
pub use pallet_authorship;

parameter_types! {
	pub const UncleGenerations: BlockNumber = 5;
}

impl pallet_authorship::Config for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    //type EventHandler = (Staking, ImOnline);
    type EventHandler = Staking;
}