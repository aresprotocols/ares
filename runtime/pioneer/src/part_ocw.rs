use super::*;
use crate::governance::part_technical::TechnicalCollective;
use ares_oracle;

use codec::Encode;
use frame_support::{
	sp_runtime::{
		generic::{Era, SignedPayload},
		traits,
	}
};
use frame_support::instances::Instance1;
use frame_support::traits::EitherOfDiverse;

// An index to a block.
pub type BlockNumber = u32;

pub type EnsureRootOrHalfTechnicalCollective = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 2>,
>;

parameter_types! {
	pub const UnsignedPriority: u64 = 1 << 20;
	pub const CalculationKind: u8 = 1;
	pub const ErrLogPoolDepth: u32 = 1000;
}

impl staking_extend::Config for Runtime {
	type AuthorityId = AresId;
}

impl ares_oracle::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type OffchainAppCrypto = ares_oracle::ares_crypto::AresCrypto<AresId>;
	type AuthorityAres = AresId;
	type UnsignedPriority = UnsignedPriority;
	// type FindAuthor = Babe;
	type CalculationKind = CalculationKind;
	type RequestOrigin = EnsureRootOrHalfTechnicalCollective;
	type AuthorityCount = AresOracle; // ares_oracle::aura_handler::Pallet<Runtime>;
	type FinanceInstance = Instance1;
	type OracleFinanceHandler = OracleFinance;
	type AresIStakingNpos = staking_extend::StakingNPOS<Self>;
	type ErrLogPoolDepth = ErrLogPoolDepth;
	type WeightInfo = ares_oracle::weights::SubstrateWeight<Self> ;
	type IOracleAvgPriceEvents = (AresReminder);
}
