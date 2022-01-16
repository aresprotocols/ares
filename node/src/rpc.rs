//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use frame_support::sp_runtime::traits::{Hash, Header};
use futures::future::{ready, TryFutureExt};
use jsonrpc_core::{
	futures::future::{result, Future},
	Error as RpcError, ErrorCode,
};
use jsonrpc_derive::rpc;
use runtime_gladios_node::{opaque::Block, AccountId, Balance, Index};
use sc_client_api::client::ProvideUncles;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::{ProvideRuntimeApi, HeaderT};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_runtime::traits::{Block as BlockT, NumberFor};
use sc_client_api::blockchain::Backend;
use sc_consensus::{BlockImport, BlockCheckParams, ImportResult};
use sc_rpc::chain::new_full;
use sc_service::TaskExecutor;
use sp_consensus::BlockOrigin;
use sp_rpc::{list::ListOrValue, number::NumberOrHex};
use sc_block_builder::BlockBuilderProvider;

use jsonrpc_pubsub::manager::SubscriptionManager;
use sc_client_api::BlockBackend;
use frame_support::sp_runtime::generic::BlockId;


/// Full client dependencies.
pub struct FullDeps<C, P, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,

	pub backend: Arc<B>
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, B>(deps: FullDeps<C, P, B>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> +BlockBackend<Block> + BlockImport<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C: ProvideUncles<Block> + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use substrate_frame_rpc_system::{FullSystem, SystemApi};

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps { client, pool, deny_unsafe, backend } = deps;

	io.extend_with(SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe)));

	io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone())));
	io.extend_with(AresApi::to_delegate(Ares::new(client.clone(), backend.clone())));
	// client.block_number_from_id()
	// Extend this RPC with a custom API by using the following syntax.
	// `YourRpcStruct` should have a reference to a client, which is needed
	// to call into the runtime.
	// `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

	io
}

type FutureResult<T> = Box<dyn Future<Item = T, Error = RpcError> + Send>;

#[rpc]
pub trait AresApi<Block, BlockHash, BlockNum> {
	#[rpc(name = "system_children", alias("system_childrenAt"))]
	fn children(
		&self,
		parent_hash: BlockHash,
	) -> FutureResult<Vec<BlockHash>>;

	#[rpc(name = "system_forkBlocks", alias("system_forkBlocksAt"))]
	fn check_block(
		&self,
		number: BlockNum,
		parent_hash: BlockHash,
	) ;
}

pub struct Ares<C, B> {
	client: Arc<C>,
	backend: Arc<B>,
}

impl<C, B> Ares<C, B> {
	/// Create new `FullSystem` given client and transaction pool.
	pub fn new(client: Arc<C>, backend: Arc<B>) -> Self {
		Ares { client, backend }
	}
}

impl<C, B, Block> AresApi<Block, <Block as BlockT>::Hash, <<Block as BlockT>::Header as HeaderT>::Number> for Ares<C, B>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + BlockBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C: ProvideUncles<Block> + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C: BlockImport<Block>,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	Block: BlockT,
{
	fn children(&self, parent_hash: <Block as BlockT>::Hash) -> FutureResult<Vec<<Block as BlockT>::Hash>> {
		let res = self.backend.blockchain().children(parent_hash);
		let res = res.map_err(|e| RpcError {
			code: ErrorCode::InternalError,
			message: "Unable to query block.".into(),
			data: Some(format!("{:?}", e).into()),
		});
		Box::new(result(res))
	}

	fn check_block(
		&self,
		number: <<Block as BlockT>::Header as HeaderT>::Number,
		hash: <Block as BlockT>::Hash) {

		// let match hash.into() {
		// 	None => self.client().info().best_hash,
		// 	Some(hash) => hash,
		// }

		// self.client.block(&BlockId::Hash(self.unwrap_or_best(hash))).unwrap().unwrap().block;
		// let params = BlockCheckParams {
		// 	hash: hash.clone(),
		// 	number: number,
		// 	parent_hash: hash.clone(), // block_ok.header().parent_hash().clone(),
		// 	allow_missing_state: false,
		// 	allow_missing_parent: false,
		// 	import_existing: false,
		// };

		// let res = self.client.check_block(params);
		// res
		// async move { res }.boxed()
	}
}


#[test]
fn should_return_a_block() {

	use substrate_test_runtime_client::{
		prelude::*,
		runtime::{Block as TestBlock, Header as TestHeader, H256},
	};

	let mut client = Arc::new(substrate_test_runtime_client::new());
	// let api = new_full(client.clone(), SubscriptionManager::new(Arc::new(TaskExecutor)));

	let block = client.new_block(Default::default()).unwrap().build().unwrap().block;
	let block_hash = block.hash();
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
	// assert_matches!(executor::block_on(api.block(Some(H256::from_low_u64_be(5)).into())), Ok(None));
}
