//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use frame_benchmarking::frame_support::sp_runtime::traits::{Hash, Header};
use futures::{future::ready, FutureExt, TryFutureExt};
use jsonrpc_core::{Error as RpcError, ErrorCode};
use jsonrpc_derive::rpc;
use runtime_gladios_node::{opaque::Block, AccountId, Balance, Index};
use sc_client_api::client::ProvideUncles;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_runtime::traits::{Block as BlockT, NumberFor};
use sc_client_api::blockchain::Backend;
use sc_consensus::{BlockImport, BlockCheckParams};

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
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
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

type FutureResult<T> = jsonrpc_core::BoxFuture<Result<T, RpcError>>;

#[rpc]
pub trait AresApi<Block, BlockHash> {
	#[rpc(name = "system_children", alias("system_childrenAt"))]
	fn children(
		&self,
		parent_hash: BlockHash,
	) -> FutureResult<Vec<BlockHash>>;

	#[rpc(name = "system_fork_blocks", alias("system_forkBlocks"))]
	fn fork_blocks(
		&self,
		parent_hash: BlockHash,
	) -> FutureResult<Vec<BlockHash>>;
}

pub struct Ares<C, B> {
	client: Arc<C>,
	backend: Arc<B>,
}

impl<C, B> Ares<C, B> {
	/// Create new `FullSystem` given client and transaction pool.
	pub fn new(client: Arc<A>, backend: Arc<B>) -> Self {
		Ares { client, backend }
	}
}

impl<C, B, Block> AresApi<Block, <Block as BlockT>::Hash> for Ares<C, B>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
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
		async move { res }.boxed()
	}

	fn fork_blocks(&mut self, number: ,hash: <Block as BlockT>::Hash) -> FutureResult<Vec<<Block as BlockT>::Hash>> {

		let params = BlockCheckParams {
			hash: hash.clone(),
			number: 0,
			parent_hash: block_ok.header().parent_hash().clone(),
			allow_missing_state: false,
			allow_missing_parent: false,
			import_existing: false,
		};

		let res = self.client.check_block();
	}
}