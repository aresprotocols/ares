use jsonrpc_derive::rpc;
use std::{convert::TryInto, sync::Arc};
use sp_blockchain::HeaderBackend;
use codec::{Decode, Encode};
use futures::{
	future::{FutureExt, TryFutureExt},
	SinkExt, StreamExt as _,
};
use jsonrpc_pubsub::{manager::SubscriptionManager, typed::Subscriber, SubscriptionId};
use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::{
	error::IntoPoolError, BlockHash, InPoolTransaction, TransactionFor, TransactionPool,
	TransactionSource, TransactionStatus, TxHash,
};
use sp_api::ProvideRuntimeApi;
use sp_core::Bytes;
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::{generic, traits::Block as BlockT};
use sp_session::SessionKeys;

use jsonrpc_core::{FutureResult, Result, ErrorCode};
/// Re-export the API for backward compatibility.
pub use sc_rpc_api::author::*;

/// Substrate authoring RPC API
#[rpc]
pub trait AresApi<Hash, BlockHash> {

	/// Insert a key into the keystore.
	#[rpc(name = "ares_insertKey")]
	fn insert_key(&self, key_type: String, suri: String, public: Bytes) -> Result<()>;

	#[rpc(name = "ares_hasKey")]
	fn has_key(&self, public_key: Bytes, key_type: String) -> Result<bool>;

}


/// Author RPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Key type ID has an unknown format.
	#[error("Invalid key type ID format (should be of length four)")]
	BadKeyType,

	/// Some random issue with the key store. Shouldn't happen.
	#[error("The key store is unavailable")]
	KeyStoreUnavailable,
}

impl From<Error> for jsonrpc_core::Error {
	fn from(e: Error) -> Self {
		jsonrpc_core::Error {
			code: jsonrpc_core::ErrorCode::InternalError,
			message: "Unknown error occurred".into(),
			data: Some(e.to_string().into()),
		}
	}
}

/// Authoring API
pub struct AresStruct<P, Client> {
	/// Substrate client
	client: Arc<Client>,
	/// Transactions pool
	pool: Arc<P>,
	/// Subscriptions manager
	subscriptions: SubscriptionManager,
	/// The key store.
	keystore: SyncCryptoStorePtr,
	/// Whether to deny unsafe calls
	deny_unsafe: DenyUnsafe,
}

impl<P, Client> AresStruct<P, Client> {
	/// Create new instance of Authoring API.
	pub fn new(
		client: Arc<Client>,
		pool: Arc<P>,
		subscriptions: SubscriptionManager,
		keystore: SyncCryptoStorePtr,
		deny_unsafe: DenyUnsafe,
	) -> Self {
		AresStruct { client, pool, subscriptions, keystore, deny_unsafe }
	}


}

/// Currently we treat all RPC transactions as externals.
///
/// Possibly in the future we could allow opt-in for special treatment
/// of such transactions, so that the block authors can inject
/// some unique transactions via RPC and have them included in the pool.
const TX_SOURCE: TransactionSource = TransactionSource::External;

impl<P, Client> AresApi<TxHash<P>, BlockHash<P>> for AresStruct<P, Client>
where
	P: TransactionPool + Sync + Send + 'static,
	Client: HeaderBackend<P::Block> + ProvideRuntimeApi<P::Block> + Send + Sync + 'static,
	Client::Api: SessionKeys<P::Block>,
	P::Hash: Unpin,
	<P::Block as BlockT>::Hash: Unpin,
{

	fn insert_key(&self, key_type: String, suri: String, public: Bytes) -> Result<()> {
		self.deny_unsafe.check_if_safe()?;

		let key_type = key_type.as_str().try_into().map_err(|_| Error::BadKeyType)?;
		SyncCryptoStore::insert_unknown(&*self.keystore, key_type, &suri, &public[..])
			.map_err(|_| Error::KeyStoreUnavailable)?;
		Ok(())
	}

	fn has_key(&self, public_key: Bytes, key_type: String) -> Result<bool> {
		self.deny_unsafe.check_if_safe()?;

		let key_type = key_type.as_str().try_into().map_err(|_| Error::BadKeyType)?;
		Ok(SyncCryptoStore::has_keys(&*self.keystore, &[(public_key.to_vec(), key_type)]))
	}

}
