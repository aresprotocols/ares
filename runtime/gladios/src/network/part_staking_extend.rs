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
    type ValidatorId = AccountId ;
    type ValidatorSet = Historical;
    type AuthorityId = AresId ;
    type AresOraclePreCheck = AresOracle;
}


impl staking_extend::elect::Config for Runtime {
    type ElectionProvider = ElectionProviderMultiPhase;
    type DataProvider = Staking;
}