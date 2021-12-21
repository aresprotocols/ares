use super::*;

/// Import the template pallet.
pub use member_extend;

///
impl member_extend::Config for Runtime {
    type MemberAuthority = AuraId ;
    type Member = Aura;
    // type ValidatorId = <Self as frame_system::Config>::AccountId;
}
