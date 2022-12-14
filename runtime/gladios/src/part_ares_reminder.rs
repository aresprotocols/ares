use super::*;
use frame_support::traits::EitherOfDiverse;
use crate::governance::part_technical::TechnicalCollective;
use frame_support::instances::Instance2;

pub type EnsureRootOrHalfTechnicalCollective = EitherOfDiverse<
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 2>,
>;

parameter_types! {
	pub const UnsignedPriorityAresReminder: u64 = 2 << 20;
}

impl ares_reminder::Config for Runtime {
    type OffchainAppCrypto = ares_oracle::ares_crypto::AresCrypto<AresId>;
    type AuthorityAres = AresId;
    type Event = Event;
    type FinanceInstance = Instance2;
    type OracleFinanceHandler = ReminderFinance;
    type PriceProvider = AresOracle;
    type RequestOrigin =EnsureRootOrHalfTechnicalCollective;
    type StashAndAuthorityPort = AresOracle;
    type UnsignedPriority = UnsignedPriorityAresReminder;
}