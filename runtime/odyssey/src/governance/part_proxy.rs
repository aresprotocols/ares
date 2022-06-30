use super::*;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::InstanceFilter;
use pallet_proxy;

parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = deposit(1, 8) * ARES_AMOUNT_MULT;
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = deposit(0, 33) * ARES_AMOUNT_MULT;
	pub const MaxProxies: u16 = 32;
	pub const AnnouncementDepositBase: Balance = deposit(1, 8) * ARES_AMOUNT_MULT;
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66) * ARES_AMOUNT_MULT;
	pub const MaxPending: u16 = 32;
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug, MaxEncodedLen, scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	// Staking,
	/// Allow to veto an announced proxy call.
	CancelProxy,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(
				c,
				Call::System(..)
				| Call::Timestamp(..)
				| Call::Council(..)
				| Call::TechnicalCommittee(..)
				| Call::Democracy(..)
				//| Call::Utility(..)
				| Call::Proxy(..)
			),
			ProxyType::Governance => matches!(
				c,
				Call::Council(..) | Call::TechnicalCommittee(..) | Call::Democracy(..) // | Call::Utility(..)
			),
			// ProxyType::Staking => matches!(
			// 	c,
			// 	// Call::Staking(..)
			// 	Call::Utility(..)
			// ),
			ProxyType::CancelProxy => {
				matches!(c, Call::Proxy(pallet_proxy::Call::reject_announcement { .. }))
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = MaxProxies;
	type WeightInfo = pallet_proxy::weights::SubstrateWeight<Runtime>;
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}
