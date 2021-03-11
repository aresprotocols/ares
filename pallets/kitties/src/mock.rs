use crate::*;
use pallet_balances as balances;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;
use frame_support::{ impl_outer_event, impl_outer_origin, parameter_types, weights::Weight};
use sp_io;

impl_outer_origin! {
	pub enum origin for TestKitty {}
}
mod kitties {
	pub use crate::Event;
}

// 导入外部的事件定义
impl_outer_event! {
	pub enum TestEvent for TestKitty {
		kitties<T>,
		system<T>,
		balances<T>,
	}
}


#[derive(Clone, Eq, PartialEq,Debug)]
pub struct TestKitty;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	pub const ExistentialDeposit: u64 = 1;
}


impl system::Trait for TestKitty {
	type BaseCallFilter = ();
	type Origin = origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = TestEvent;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

impl balances::Trait for TestKitty {
	type Balance = u64;
	type MaxLocks = ();
	type Event = TestEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = system::Module<TestKitty>;
	type WeightInfo = ();
}

parameter_types! {
	pub const LockAmount: u64 = 5_000;
}

//实现kitty的pallet里的trait
impl Trait for TestKitty {
	//type Event = ();
	type Event = TestEvent;
	type Randomness = Randomness;
	type KittyIndex = u32;
	type Currency = balances::Module<Self>;
	type LockAmount = LockAmount;
	//type Currency = balances::Module<Self>;
}

pub type Kitties = Module<TestKitty>;

pub type System = frame_system::Module<TestKitty>;

pub fn new_test_ext() -> sp_io::TestExternalities{


	let mut t = system::GenesisConfig::default()
		.build_storage::<TestKitty>()
		.unwrap();
	balances::GenesisConfig::<TestKitty> {
		// Provide some initial balances
		balances: vec![(1, 10000000000), (2, 110000000), (3, 1200000000), (4, 1300000000), (5, 1400000000)],
	}
		.assimilate_storage(&mut t)
		.unwrap();
	let mut ext: sp_io::TestExternalities = t.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}

type Randomness = pallet_randomness_collective_flip::Module<TestKitty>;