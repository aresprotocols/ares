use super::*;


impl staking_extend::Config for Runtime {
    type ValidatorId = AccountId ;
    type ValidatorSet = Historical;
    type AuthorityId = AresId ;
    type DataProvider = Staking;
    type ElectionProvider = ElectionProviderMultiPhase;
    type OnChainAccuracy = Perbill;
    type AresOraclePreCheck = AresOracle;
}