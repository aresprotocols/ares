use frame_system::EnsureSigned;
use sp_runtime::traits::ConstU32;
use crate::*;

parameter_types! {
	// pub const AlarmInterval: BlockNumber = 1;
	pub const SubmissionDeposit: Balance = 100 * DOLLARS;
	// pub const UndecidingTimeout: BlockNumber = 28 * DAYS;
}

impl pallet_referenda::Config for Runtime {
    type WeightInfo = pallet_referenda::weights::SubstrateWeight<Self>;
    type Call = Call;
    type Event = Event;
    type Scheduler = Scheduler;
    type Currency = pallet_balances::Pallet<Self>;
    type SubmitOrigin = EnsureSigned<AccountId>;
    type CancelOrigin = EnsureRoot<AccountId>;
    type KillOrigin = EnsureRoot<AccountId>;
    type Slash = ();
    type Votes = pallet_conviction_voting::VotesOf<Runtime>;
    type Tally = pallet_conviction_voting::TallyOf<Runtime>;
    type SubmissionDeposit = SubmissionDeposit;
    type MaxQueued = ConstU32<100>;
    type UndecidingTimeout = UndecidingTimeout;
    type AlarmInterval = AlarmInterval;
    type Tracks = TracksInfo;
}