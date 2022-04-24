use jsonrpc_pubsub::manager::SubscriptionManager;
use sc_rpc::author::Author;
use sp_keystore::testing::KeyStore;
use substrate_test_runtime_client::{
    self,
    runtime::{Block, Extrinsic, SessionKeys, Transfer},
    AccountKeyring, Backend, Client, DefaultTestClientBuilderExt, TestClientBuilderExt,
};

use sp_core::{
    blake2_256,
    crypto::{ByteArray, CryptoTypePublicPair, Pair},
    ed25519,
    hexdisplay::HexDisplay,
    sr25519,
    testing::{ED25519, SR25519},
    H256,
};

use std::{mem, sync::Arc};
use sc_rpc_api::DenyUnsafe;

struct TestSetup {
    pub client: Arc<Client<Backend>>,
    pub keystore: Arc<KeyStore>,
    pub pool: Arc<FullTransactionPool>,
}

impl Default for TestSetup {
    fn default() -> Self {
        let keystore = Arc::new(KeyStore::new());
        let client_builder = substrate_test_runtime_client::TestClientBuilder::new();
        let client = Arc::new(client_builder.set_keystore(keystore.clone()).build());

        let spawner = sp_core::testing::TaskExecutor::new();
        let pool =
            BasicPool::new_full(Default::default(), true.into(), None, spawner, client.clone());
        TestSetup { client, keystore, pool }
    }
}

impl TestSetup {
    fn author(&self) -> AresStruct<FullTransactionPool, Client<Backend>> {
        AresStruct {
            client: self.client.clone(),
            pool: self.pool.clone(),
            subscriptions: SubscriptionManager::new(Arc::new(crate::testing::TaskExecutor)),
            keystore: self.keystore.clone(),
            deny_unsafe: DenyUnsafe::No,
        }
    }
}

#[test]
fn should_insert_key() {
    let setup = TestSetup::default();
    let p = setup.author();

    let suri = "//Alice";
    let key_pair = ed25519::Pair::from_string(suri, None).expect("Generates keypair");
    p.insert_key(
        String::from_utf8(ED25519.0.to_vec()).expect("Keytype is a valid string"),
        suri.to_string(),
        key_pair.public().0.to_vec().into(),
    )
        .expect("Insert key");

    let public_keys = SyncCryptoStore::keys(&*setup.keystore, ED25519).unwrap();

    assert!(public_keys
        .contains(&CryptoTypePublicPair(ed25519::CRYPTO_ID, key_pair.public().to_raw_vec())));
}

#[test]
fn test_has_key() {
    let setup = TestSetup::default();
    let p = setup.author();

    let suri = "//Alice";
    let alice_key_pair = ed25519::Pair::from_string(suri, None).expect("Generates keypair");
    p.insert_key(
        String::from_utf8(ED25519.0.to_vec()).expect("Keytype is a valid string"),
        suri.to_string(),
        alice_key_pair.public().0.to_vec().into(),
    )
        .expect("Insert key");
    let bob_key_pair = ed25519::Pair::from_string("//Bob", None).expect("Generates keypair");

    let test_vectors = vec![
        (alice_key_pair.public().to_raw_vec().into(), ED25519, Ok(true)),
        (alice_key_pair.public().to_raw_vec().into(), SR25519, Ok(false)),
        (bob_key_pair.public().to_raw_vec().into(), ED25519, Ok(false)),
    ];

    for (key, key_type, result) in test_vectors {
        assert_eq!(
            result.map_err(|e| mem::discriminant(&e)),
            p.has_key(
                key,
                String::from_utf8(key_type.0.to_vec()).expect("Keytype is a valid string"),
            )
                .map_err(|e| mem::discriminant(&e)),
        );
    }
}
