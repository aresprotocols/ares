use super::*;
use frame_election_provider_support::{ElectionProvider, onchain};
use frame_support::traits::OneSessionHandler;
use ares_oracle::crypto2::AuraAuthId;

impl staking_extend::Config for Runtime {
    type ValidatorId = AccountId ; //<Self as frame_system::Config>::AccountId;
    type ValidatorSet = Historical;
    // type AuthorityId = <Aura as OneSessionHandler<AccountId>>::Key;
    type AuthorityId = AuraId ;// <OCWModule as ares_oracle::Config>::AuthorityAres ;// AuraId;
    // type WithSessionHandler = Aura;
    // type IStakingNpos = Self;
    // type DebugError = <<Self as staking_extend::Config>::ElectionProvider as ElectionProvider<<Self as frame_system::Config>::AccountId, <Self as frame_system::Config>::BlockNumber>>::Error;
    type DataProvider = Staking;
    type ElectionProvider = ElectionProviderMultiPhase;
    type OnChainAccuracy = Perbill;
    type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
        pallet_election_provider_multi_phase::OnChainConfig<Self>,
    >;
    type AresOraclePreCheck = OCWModule;
}