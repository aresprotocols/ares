use frame_support::{pallet_prelude::Encode, sp_std};
use log;
use runtime_pioneer_node::{
	self, api::dispatch, native_version, opaque::Block, part_ocw::LOCAL_STORAGE_PRICE_REQUEST_DOMAIN, RuntimeApi,
};
use sc_client_api::{Backend, ExecutorProvider};
// use sc_consensus_aura::{ImportQueueParams, SlotProportion, StartAuraParams};
use sc_consensus_babe::{SlotProportion};
use sc_executor::NativeElseWasmExecutor;
use sc_finality_grandpa::SharedVoterState;
use sc_keystore::LocalKeystore;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use seed_reader::*;
use sp_consensus::SlotData;
// use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use sp_core::offchain::OffchainStorage;
use sp_offchain::STORAGE_PREFIX;
use std::{io::Read, sync::Arc, time::Duration};

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		native_version()
	}
}

type FullClient = sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
