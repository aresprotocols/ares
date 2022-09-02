//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use ares_rpc::ares::{AresToolsApi, AresToolsStruct};
use futures::FutureExt;
use sp_consensus_babe::BabeApi;
use jsonrpc_core::{Error as RpcError, ErrorCode};
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::manager::SubscriptionManager;
use sc_client_api::{blockchain::Backend, client::ProvideUncles, BlockBackend, AuxStore};
use sc_consensus::BlockImport;
use sc_finality_grandpa::{FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState};
use sc_finality_grandpa_rpc::Grandpa;
// use sc_finality_grandpa_rpc::GrandpaRpcHandler;
use sc_rpc::SubscriptionTaskExecutor;
use sc_rpc::system::System;
pub use sc_rpc_api::DenyUnsafe;
use sc_service::{Configuration, Role, SpawnTaskHandle};
use sc_sync_state_rpc::SyncState;
use sc_transaction_pool_api::TransactionPool;
use sp_api::{HeaderT, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_core::H256;
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};
use sp_runtime::traits::Block as BlockT;
use sc_consensus_babe::{Config, Epoch};
use runtime_common::{AccountId, Balance, Hash};
use sc_consensus_epochs::SharedEpochChanges;

pub type Index = u32;

// use sc_service::TaskExecutor;
use crate::service::{Block, BlockNumber};

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: Config,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: SyncCryptoStorePtr,
}

/// Extra dependencies for GRANDPA
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<H256, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
}

// -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
/// Instantiate all full RPC extensions.
pub fn create_full<C, P, SC, B>(
	deps: FullDeps<C, P, SC, B>,
	backend: Arc<B>,
)
	// -> jsonrpc_core::IoHandler<sc_rpc::Metadata> Old
	-> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
	+ sc_client_api::BlockBackend<Block>
	+ HeaderBackend<Block>
	+ AuxStore
	+ HeaderMetadata<Block, Error = BlockChainError>
	+ Sync
	+ Send
	+ 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	// C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber, Hash>,
	// C::Api: pallet_mmr_rpc::MmrRuntimeApi<Block, <Block as sp_runtime::traits::Block>::Hash>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BabeApi<Block>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
	SC: SelectChain<Block> + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
	// use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	// use substrate_frame_rpc_system::{FullSystem, SystemApi};

	// use pallet_contracts_rpc::{Contracts, ContractsApiServer};
	// use pallet_mmr_rpc::{Mmr, MmrApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use sc_consensus_babe_rpc::{Babe, BabeApiServer};
	use sc_finality_grandpa_rpc::{Grandpa, GrandpaApiServer};
	use sc_rpc::dev::{Dev, DevApiServer};
	use sc_sync_state_rpc::{SyncState, SyncStateApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};
	use substrate_state_trie_migration_rpc::{StateMigration, StateMigrationApiServer};

	// let mut io = jsonrpc_core::IoHandler::default();
	let mut io = RpcModule::new(());

	let FullDeps { client, pool, select_chain, chain_spec, deny_unsafe, babe, grandpa } = deps;

	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;
	let BabeDeps { keystore, babe_config, shared_epoch_changes } = babe;

	// ----------------
	// io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool.clone(), deny_unsafe.clone())));
	// io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));
	// // io.extend_with(AresApi::to_delegate(Ares::new(client.clone(), backend.clone())));
	// io.extend_with(sc_finality_grandpa_rpc::GrandpaApi::to_delegate(GrandpaRpcHandler::new(
	// 	shared_authority_set.clone(),
	// 	shared_voter_state,
	// 	justification_stream,
	// 	subscription_executor,
	// 	finality_provider,
	// )));
	//
	// if let Some(offchain_storage) = backend.clone().offchain_storage() {
	// 	io.extend_with(AresToolsApi::to_delegate(AresToolsStruct::new(offchain_storage, deny_unsafe.clone(), role)));
	// }
	//------------------
	// TODO:: check some rpc.
	io.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	// Making synchronous calls in light client freezes the browser currently,
	// more context: https://github.com/paritytech/substrate/pull/3480
	// These RPCs should use an asynchronous caller instead.
	// io.merge(Contracts::new(client.clone()).into_rpc())?;
	// io.merge(Mmr::new(client.clone()).into_rpc())?;
	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	// io.merge(
	// 	Babe::new(
	// 		client.clone(),
	// 		shared_epoch_changes.clone(),
	// 		keystore,
	// 		babe_config,
	// 		select_chain,
	// 		deny_unsafe,
	// 	)
	// 		.into_rpc(),
	// )?;
	io.merge(
		Grandpa::new(
			subscription_executor,
			shared_authority_set.clone(),
			shared_voter_state,
			justification_stream,
			finality_provider,
		)
			.into_rpc(),
	)?;

	io.merge(
		SyncState::new(chain_spec, client.clone(), shared_authority_set, shared_epoch_changes)?
			.into_rpc(),
	)?;
	// TODO::Check StateMigration.
	// io.merge(StateMigration::new(client.clone(), backend, deny_unsafe).into_rpc())?;
	// TODO::Check Dev.
	// io.merge(Dev::new(client, deny_unsafe).into_rpc())?;

	Ok(io)
}

type FutureResult<T> = jsonrpc_core::BoxFuture<Result<T, RpcError>>;

#[rpc]
pub trait AresApi<Block, BlockHash, BlockNum> {
	#[rpc(name = "system_children", alias("system_childrenAt"))]
	fn children(&self, parent_hash: BlockHash) -> FutureResult<Vec<BlockHash>>;

	// #[rpc(name = "system_forkBlocks", alias("system_forkBlocksAt"))]
	// fn check_block(&self, number: BlockNum, parent_hash: BlockHash);
}

///A struct that implements the [`AresApi`].
pub struct Ares<C, B> {
	_client: Arc<C>,
	backend: Arc<B>,
}

// impl<C, B> Ares<C, B> {
// 	/// Create new `FullSystem` given client and transaction pool.
// 	pub fn new(client: Arc<C>, backend: Arc<B>) -> Self {
// 		Ares { _client: client, backend }
// 	}
// }
//
// impl<C, B, Block> AresApi<Block, <Block as BlockT>::Hash, <<Block as BlockT>::Header as HeaderT>::Number> for Ares<C, B>
// where
// 	C: ProvideRuntimeApi<Block>,
// 	C: HeaderBackend<Block> + BlockBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
// 	C: Send + Sync + 'static,
// 	C: ProvideUncles<Block> + 'static,
// 	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
// 	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
// 	C::Api: BlockBuilder<Block>,
// 	C: BlockImport<Block>,
// 	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
// 	Block: BlockT,
// {
// 	fn children(&self, parent_hash: <Block as BlockT>::Hash) -> FutureResult<Vec<<Block as BlockT>::Hash>> {
// 		let res = self.backend.blockchain().children(parent_hash);
// 		let res = res.map_err(|e| RpcError {
// 			code: ErrorCode::InternalError,
// 			message: "Unable to query block.".into(),
// 			data: Some(format!("{:?}", e).into()),
// 		});
// 		async move { res }.boxed()
// 	}
//
// 	// fn check_block(&self, number: <<Block as BlockT>::Header as HeaderT>::Number, hash: <Block as
// 	// BlockT>::Hash) {
//
// 	// let match hash.into() {
// 	// 	None => self.client().info().best_hash,
// 	// 	Some(hash) => hash,
// 	// }
//
// 	// self.client.block(&BlockId::Hash(self.unwrap_or_best(hash))).unwrap().unwrap().block;
// 	// let params = BlockCheckParams {
// 	// 	hash: hash.clone(),
// 	// 	number: number,
// 	// 	parent_hash: hash.clone(), // block_ok.header().parent_hash().clone(),
// 	// 	allow_missing_state: false,
// 	// 	allow_missing_parent: false,
// 	// 	import_existing: false,
// 	// };
//
// 	// let res = self.client.check_block(params);
// 	// res
// 	// async move { res }.boxed()
// 	// }
// }

#[test]
fn should_return_a_block() {
	use substrate_test_runtime_client::{
		prelude::*,
		runtime::{Block as TestBlock, Header as TestHeader, H256},
	};

	// let mut client = Arc::new(substrate_test_runtime_client::new());
	// let api = new_full(client.clone(), SubscriptionManager::new(Arc::new(TaskExecutor)));

	// let block = client.new_block(Default::default()).unwrap().build().unwrap().block;
	// let block_hash = block.hash();
	// executor::block_on(client.import(BlockOrigin::Own, block)).unwrap();

	// // Genesis block is not justified
	// assert_matches!(
	// 	executor::block_on(api.block(Some(client.genesis_hash()).into())),
	// 	Ok(Some(SignedBlock { justifications: None, .. }))
	// );
	//
	// assert_matches!(
	// 	executor::block_on(api.block(Some(block_hash).into())),
	// 	Ok(Some(ref x)) if x.block == Block {
	// 		header: Header {
	// 			parent_hash: client.genesis_hash(),
	// 			number: 1,
	// 			state_root: x.block.header.state_root.clone(),
	// 			extrinsics_root:
	// 				"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314".parse().unwrap(),
	// 			digest: Default::default(),
	// 		},
	// 		extrinsics: vec![],
	// 	}
	// );
	//
	// assert_matches!(
	// 	executor::block_on(api.block(None.into())),
	// 	Ok(Some(ref x)) if x.block == Block {
	// 		header: Header {
	// 			parent_hash: client.genesis_hash(),
	// 			number: 1,
	// 			state_root: x.block.header.state_root.clone(),
	// 			extrinsics_root:
	// 				"03170a2e7597b7b7e3d84c05391d139a62b157e78786d8c082f29dcf4c111314".parse().unwrap(),
	// 			digest: Default::default(),
	// 		},
	// 		extrinsics: vec![],
	// 	}
	// );
	//
	// assert_matches!(executor::block_on(api.block(Some(H256::from_low_u64_be(5)).into())),
	// Ok(None));
}
