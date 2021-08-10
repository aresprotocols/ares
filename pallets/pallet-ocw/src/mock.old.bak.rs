use codec::{alloc::sync::Arc};
use frame_support::{impl_outer_event, impl_outer_origin, parameter_types, weights::Weight};
use frame_system::offchain;
use parking_lot::RwLock;
use sp_core::{
    H256,
    offchain::{
        OffchainExt,
        testing::{self, OffchainState, PoolState}, TransactionPoolExt,
    },
    sr25519::{self, Signature},
    testing::KeyStore,
    traits::KeystoreExt,
};
use sp_io::TestExternalities;
use sp_runtime::{
    Perbill,
    testing::{Header, TestXt},
    traits::{BlakeTwo256, IdentityLookup, Verify},
};

use crate::*;
use crate as pallet_ocw;

impl_outer_origin! {
	pub enum Origin for Test {}
}

impl_outer_event! {
	pub enum TestEvent for Test {
		frame_system<T>,
		pallet_ocw<T>,
	}
}

// Configure a mock runtime to test the pallet.

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Trait for Test {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Index = u64;
    type Call = ();
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = sr25519::Public;
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
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}

type TestExtrinsic = TestXt<Call<Test>, ()>;

parameter_types! {
	pub const GracePeriod: u64 = 5;
}

impl Trait for Test {
    type Event = TestEvent;
    type AuthorityId = crypto::TestAuthId;
    type Call = Call<Test>;
    type GracePeriod = GracePeriod;
}

pub type System = frame_system::Module<Test>;
pub type OCWModule = Module<Test>;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

impl<LocalCall> offchain::CreateSignedTransaction<LocalCall> for Test
    where
        Call<Test>: From<LocalCall>,
{
    fn create_transaction<C: offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call<Test>,
        _public: <Signature as Verify>::Signer,
        _account: <Test as frame_system::Trait>::AccountId,
        index: <Test as frame_system::Trait>::Index,
    ) -> Option<(
        Call<Test>,
        <TestExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
    )> {
        Some((call, (index, ())))
    }
}

impl offchain::SigningTypes for Test {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
}

impl<C> offchain::SendTransactionTypes<C> for Test
    where
        Call<Test>: From<C>,
{
    type OverarchingCall = Call<Test>;
    type Extrinsic = TestExtrinsic;
}

pub struct ExternalityBuilder;

impl ExternalityBuilder {
    pub fn build() -> (
        TestExternalities,
        Arc<RwLock<PoolState>>,
        Arc<RwLock<OffchainState>>,
    ) {
        const PHRASE: &str =
            "expire stage crawl shell boss any story swamp skull yellow bamboo copy";

        let (offchain, offchain_state) = testing::TestOffchainExt::new();
        let (pool, pool_state) = testing::TestTransactionPoolExt::new();
        let keystore = KeyStore::new();
        keystore
            .write()
            .sr25519_generate_new(KEY_TYPE, Some(&format!("{}/hunter1", PHRASE)))
            .unwrap();

        let storage = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut t = TestExternalities::from(storage);
        t.register_extension(OffchainExt::new(offchain));
        t.register_extension(TransactionPoolExt::new(pool));
        t.register_extension(KeystoreExt(keystore));
        t.execute_with(|| System::set_block_number(1));
        (t, pool_state, offchain_state)
    }
}