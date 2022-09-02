use frame_election_provider_support::onchain;
use crate::network::part_elections::OnChainSeqPhragmen;
use super::*;

// impl staking_extend::Config for Runtime {
//     type ValidatorId = AccountId ;
//     type ValidatorSet = Historical;
//     type AuthorityId = AresId ;
//     type DataProvider = Staking;
//     type ElectionProvider = ElectionProviderMultiPhase;
//     type OnChainAccuracy = Perbill;
//     type AresOraclePreCheck = AresOracle;
// }

impl staking_extend::data::Config for Runtime {
	type DataProvider = Staking;
	type ValidatorId = AccountId;
	type ValidatorSet = Historical;
	type AuthorityId = AresId;
	type AresOraclePreCheck = AresOracle;
}

impl staking_extend::elect::Config for Runtime {
	// type GenesisElectionProvider = frame_election_provider_support::onchain::OnChainSequentialPhragmen<Self>;
	type GenesisElectionProvider = onchain::UnboundedExecution<OnChainSeqPhragmen>;
	type ElectionProvider = ElectionProviderMultiPhase;
	type DataProvider = Staking;
}
