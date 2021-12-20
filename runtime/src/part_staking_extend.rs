use super::*;
use frame_election_provider_support::{ElectionProvider, onchain};
use frame_support::traits::OneSessionHandler;

impl staking_extend::Config for Runtime {
    type ValidatorId = AccountId ;
    type ValidatorSet = Historical;
    type AuthorityId = AresId ;
    type DataProvider = Staking;
    type ElectionProvider = ElectionProviderMultiPhase;
    type OnChainAccuracy = Perbill;
    type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
        pallet_election_provider_multi_phase::OnChainConfig<Self>,
    >;
    type AresOraclePreCheck = AresOracle;
}