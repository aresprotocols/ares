// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Setup code for [`super::command`] which would otherwise bloat that module.
//!
//! Should only be used for benchmarking as it may break in other contexts.

use crate::service::{create_extrinsic, FullClient, RuntimeApiCollection, Block, FullBackend};

use kitchensink_runtime::{BalancesCall, SystemCall};
use sp_api::ConstructRuntimeApi;
// use node_primitives::{AccountId, Balance};
use runtime_common::{AccountId, Balance, Hash};
use sc_cli::Result;
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::OpaqueExtrinsic;
// use sp_runtime::traits::Block;

use std::{sync::Arc, time::Duration};

/// Generates `System::Remark` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct RemarkBuilder<RuntimeApi, ExecutorDispatch>
	where
		// RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>> + Send + Sync + 'static,
		// RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
		ExecutorDispatch: sc_executor::NativeExecutionDispatch + 'static,
{
	client: Arc<FullClient<RuntimeApi, ExecutorDispatch>>,
}

impl <RuntimeApi, ExecutorDispatch> RemarkBuilder<RuntimeApi, ExecutorDispatch>
	where
		RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>> + Send + Sync + 'static,
		RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
		ExecutorDispatch: sc_executor::NativeExecutionDispatch + 'static,
{
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<FullClient<RuntimeApi, ExecutorDispatch>>) -> Self {
		Self { client }
	}
}

//
impl <RuntimeApi, ExecutorDispatch> frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder<RuntimeApi, ExecutorDispatch>
	where
		RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>> + Send + Sync + 'static,
		RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
		ExecutorDispatch: sc_executor::NativeExecutionDispatch + 'static,
{
	fn pallet(&self) -> &str {
		"system"
	}

	fn extrinsic(&self) -> &str {
		"remark"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_extrinsic(
			self.client.as_ref(),
			acc,
			SystemCall::remark { remark: vec![] },
			Some(nonce),
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Balances::TransferKeepAlive` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct TransferKeepAliveBuilder<RuntimeApi, ExecutorDispatch>
	where ExecutorDispatch: sc_executor::NativeExecutionDispatch + 'static,
{
	client: Arc<FullClient<RuntimeApi, ExecutorDispatch>>,
	dest: AccountId,
	value: Balance,
}


impl <RuntimeApi, ExecutorDispatch> TransferKeepAliveBuilder<RuntimeApi, ExecutorDispatch> where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	ExecutorDispatch: sc_executor::NativeExecutionDispatch + 'static,
{
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<FullClient<RuntimeApi, ExecutorDispatch>>, dest: AccountId, value: Balance) -> Self {
		Self { client, dest, value }
	}
}

impl <RuntimeApi, ExecutorDispatch> frame_benchmarking_cli::ExtrinsicBuilder for TransferKeepAliveBuilder<RuntimeApi, ExecutorDispatch> where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	ExecutorDispatch: sc_executor::NativeExecutionDispatch + 'static,
{
	fn pallet(&self) -> &str {
		"balances"
	}

	fn extrinsic(&self) -> &str {
		"transfer_keep_alive"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_extrinsic(
			self.client.as_ref(),
			acc,
			BalancesCall::transfer_keep_alive {
				dest: self.dest.clone().into(),
				value: self.value.into(),
			},
			Some(nonce),
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates inherent data for the `benchmark overhead` command.
pub fn inherent_benchmark_data() -> Result<InherentData> {
	let mut inherent_data = InherentData::new();
	let d = Duration::from_millis(0);
	let timestamp = sp_timestamp::InherentDataProvider::new(d.into());

	timestamp
		.provide_inherent_data(&mut inherent_data)
		.map_err(|e| format!("creating inherent data: {:?}", e))?;
	Ok(inherent_data)
}