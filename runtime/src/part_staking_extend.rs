use super::*;
use frame_election_provider_support::{ElectionProvider, onchain};

impl staking_extend::Config for Runtime {
    type ValidatorId = <Self as frame_system::Config>::AccountId;
    type ValidatorSet = Historical;
    type DataProvider = Staking;
    // type DebugError = <<Self as staking_extend::Config>::ElectionProvider as ElectionProvider<<Self as frame_system::Config>::AccountId, <Self as frame_system::Config>::BlockNumber>>::Error;
    type ElectionProvider = ElectionProviderMultiPhase;
    type OnChainAccuracy = Perbill;
    type GenesisElectionProvider = onchain::OnChainSequentialPhragmen<
        pallet_election_provider_multi_phase::OnChainConfig<Self>,
    >;
}