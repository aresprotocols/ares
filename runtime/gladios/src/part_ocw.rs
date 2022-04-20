use super::*;
use crate::governance::part_technical::TechnicalCollective;
use ares_oracle;
pub use ares_oracle::LOCAL_STORAGE_PRICE_REQUEST_DOMAIN;
use codec::Encode;
use frame_support::{
	sp_runtime::{
		generic::{Era, SignedPayload},
		traits,
	},
	traits::EnsureOneOf,
};
use sp_runtime::{MultiAddress, SaturatedConversion};

// An index to a block.
pub type BlockNumber = u32;

pub type EnsureRootOrHalfTechnicalCollective = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, TechnicalCollective>,
>;

parameter_types! {
	pub const UnsignedPriority: u64 = 1 << 20;
	pub const CalculationKind: u8 = 1;
	pub const ErrLogPoolDepth: u32 = 1000;
}

// // impl ares_oracle::aura_handler::Config for Runtime {}
// impl ares_oracle::babe_handler::Config for Runtime {
// 	type AuthorityId = pallet_babe::AuthorityId;
// }

impl staking_extend::Config for Runtime {
	type AuthorityId = AresId;
}

impl ares_oracle::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type OffchainAppCrypto = ares_oracle::AresCrypto<AresId>;
	type AuthorityAres = AresId;
	type UnsignedPriority = UnsignedPriority;
	type FindAuthor = Babe;
	type CalculationKind = CalculationKind;
	type RequestOrigin = EnsureRootOrHalfTechnicalCollective;
	type AuthorityCount = AresOracle; // ares_oracle::aura_handler::Pallet<Runtime>;
	type OracleFinanceHandler = OracleFinance;
	type AresIStakingNpos = staking_extend::StakingNPOS<Self>;
	type ErrLogPoolDepth = ErrLogPoolDepth;
}

