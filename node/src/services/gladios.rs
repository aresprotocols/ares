use frame_support::{pallet_prelude::Encode, sp_std};
use log;
use runtime_gladios_node::{
	self, api::dispatch, native_version, opaque::Block, part_ocw::LOCAL_STORAGE_PRICE_REQUEST_DOMAIN, RuntimeApi,
};
use sc_client_api::{Backend, ExecutorProvider};
use sc_consensus_aura::{ImportQueueParams, SlotProportion, StartAuraParams};
use sc_executor::NativeElseWasmExecutor;
use sc_finality_grandpa::SharedVoterState;
use sc_keystore::LocalKeystore;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use seed_reader::*;
use sp_consensus::SlotData;
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
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

pub fn new_partial(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>,
			sc_finality_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
			Option<Telemetry>,
		),
	>,
	ServiceError,
> {
	if config.keystore_remote.is_some() {
		return Err(ServiceError::Other(format!("Remote Keystores are not supported.")))
	}

	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
	);

	let (client, backend, keystore_container, task_manager) = sc_service::new_full_parts::<Block, RuntimeApi, _>(
		&config,
		telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
		executor,
	)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let slot_duration = sc_consensus_aura::slot_duration(&*client)?.slot_duration();

	let import_queue = sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _, _>(ImportQueueParams {
		block_import: grandpa_block_import.clone(),
		justification_import: Some(Box::new(grandpa_block_import.clone())),
		client: client.clone(),
		create_inherent_data_providers: move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
				*timestamp,
				slot_duration,
			);

			Ok((timestamp, slot))
		},
		spawner: &task_manager.spawn_essential_handle(),
		can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
		registry: config.prometheus_registry(),
		check_for_equivocation: Default::default(),
		telemetry: telemetry.as_ref().map(|x| x.handle()),
	})?;

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (grandpa_block_import, grandpa_link, telemetry),
	})
}

fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
	// FIXME: here would the concrete keystore be built,
	//        must return a concrete type (NOT `LocalKeystore`) that
	//        implements `CryptoStore` and `SyncCryptoStore`
	Err("Remote Keystore not supported.")
}

// /// Builds a new service for a full client.
// pub fn new_full(
// 	mut config: Configuration,
// 	ares_params: Vec<(&str, Option<Vec<u8>>)>,
// ) -> Result<TaskManager, ServiceError> {
// 	let sc_service::PartialComponents {
// 		client,
// 		backend,
// 		mut task_manager,
// 		import_queue,
// 		mut keystore_container,
// 		select_chain,
// 		transaction_pool,
// 		other: (block_import, grandpa_link, mut telemetry),
// 	} = new_partial(&config)?;
//
// 	if let Some(url) = &config.keystore_remote {
// 		match remote_keystore(url) {
// 			Ok(k) => keystore_container.set_remote_keystore(k),
// 			Err(e) => return Err(ServiceError::Other(format!("Error hooking up remote keystore for {}: {}", url, e))),
// 		};
// 	}
//
// 	config.network.extra_sets.push(sc_finality_grandpa::grandpa_peers_set_config());
// 	let warp_sync = Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
// 		backend.clone(),
// 		grandpa_link.shared_authority_set().clone(),
// 		Vec::default(),
// 	));
//
// 	let (network, system_rpc_tx, network_starter) = sc_service::build_network(sc_service::BuildNetworkParams {
// 		config: &config,
// 		client: client.clone(),
// 		transaction_pool: transaction_pool.clone(),
// 		spawn_handle: task_manager.spawn_handle(),
// 		import_queue,
// 		block_announce_validator_builder: None,
// 		warp_sync: Some(warp_sync),
// 	})?;
//
// 	if config.offchain_worker.enabled {
// 		sc_service::build_offchain_workers(&config, task_manager.spawn_handle(), client.clone(), network.clone());
// 	}
//
// 	let role = config.role.clone();
// 	let force_authoring = config.force_authoring;
// 	let backoff_authoring_blocks: Option<()> = None;
// 	let name = config.network.node_name.clone();
// 	let enable_grandpa = !config.disable_grandpa;
// 	let prometheus_registry = config.prometheus_registry().cloned();
//
// 	let rpc_extensions_builder = {
// 		let client = client.clone();
// 		let pool = transaction_pool.clone();
// 		let backend = backend.clone();
// 		Box::new(move |deny_unsafe, _| {
// 			let deps = crate::rpc::FullDeps {
// 				client: client.clone(),
// 				pool: pool.clone(),
// 				deny_unsafe,
// 				backend: backend.clone(),
// 			};
//
// 			Ok(crate::rpc::create_full(deps))
// 		})
// 	};
//
// 	let backend_clone = backend.clone();
// 	let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
// 		network: network.clone(),
// 		client: client.clone(),
// 		keystore: keystore_container.sync_keystore(),
// 		task_manager: &mut task_manager,
// 		transaction_pool: transaction_pool.clone(),
// 		rpc_extensions_builder,
// 		backend,
// 		system_rpc_tx,
// 		config,
// 		telemetry: telemetry.as_mut(),
// 	})?;
//
// 	let result: Vec<(&str, bool)> = ares_params
// 		.iter()
// 		.map(|(order, x)| {
// 			match order {
// 				&"warehouse" => {
// 					match x {
// 						None => (*order, false),
// 						Some(exe_vecu8) => {
// 							let request_base_str = sp_std::str::from_utf8(exe_vecu8).unwrap();
// 							let store_request_u8 = request_base_str.encode();
// 							// let store_request_u8 = request_base_str.as_bytes();
// 							log::info!("setting request_domain: {:?}", request_base_str);
// 							if let Some(mut offchain_db) = backend_clone.offchain_storage() {
// 								log::debug!("after setting request_domain: {:?}", request_base_str);
// 								offchain_db.set(
// 									STORAGE_PREFIX,
// 									LOCAL_STORAGE_PRICE_REQUEST_DOMAIN, // copy from ocw-suit
// 									store_request_u8.as_slice(),
// 								);
// 							}
// 							(*order, true)
// 						},
// 					}
// 				},
// 				&"ares-keys-file" => {
// 					match x {
// 						None => (*order, false),
// 						Some(exe_vecu8) => {
// 							let key_file_path = sp_std::str::from_utf8(exe_vecu8).unwrap();
// 							let mut file = std::fs::File::open(key_file_path).unwrap();
// 							let mut contents = String::new();
// 							file.read_to_string(&mut contents).unwrap();
// 							let rawkey_list = extract_content(contents.as_str());
// 							let insert_key_list: Vec<(&str, &str, String)> =
// 								rawkey_list.iter().map(|x| make_author_insert_key_params(*x)).collect();
// 							let rpc_list: Vec<Option<String>> = insert_key_list
// 								.iter()
// 								.map(|x| make_rpc_request("author_insertKey", (x.0, x.1, x.2.as_str())))
// 								.collect();
// 							rpc_list.iter().any(|x| {
// 								if let Some(rpc_str) = x {
// 									// send rpc request.
// 									_rpc_handlers
// 										.io_handler()
// 										.handle_request_sync(rpc_str, sc_rpc::Metadata::default());
// 								}
// 								false
// 							});
//
// 							(*order, true)
// 						},
// 					}
// 				},
// 				&_ => ("NONE", false),
// 			}
// 		})
// 		.collect();
//
// 	if role.is_authority() {
// 		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
// 			task_manager.spawn_handle(),
// 			client.clone(),
// 			transaction_pool,
// 			prometheus_registry.as_ref(),
// 			telemetry.as_ref().map(|x| x.handle()),
// 		);
//
// 		let can_author_with = sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());
//
// 		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
// 		let raw_slot_duration = slot_duration.slot_duration();
//
// 		let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _, _>(StartAuraParams {
// 			slot_duration,
// 			client: client.clone(),
// 			select_chain,
// 			block_import,
// 			proposer_factory,
// 			create_inherent_data_providers: move |_, ()| async move {
// 				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
//
// 				let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
// 					*timestamp,
// 					raw_slot_duration,
// 				);
//
// 				Ok((timestamp, slot))
// 			},
// 			force_authoring,
// 			backoff_authoring_blocks,
// 			keystore: keystore_container.sync_keystore(),
// 			can_author_with,
// 			sync_oracle: network.clone(),
// 			justification_sync_link: network.clone(),
// 			block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
// 			max_block_proposal_slot_portion: None,
// 			telemetry: telemetry.as_ref().map(|x| x.handle()),
// 		})?;
//
// 		// the AURA authoring task is considered essential, i.e. if it
// 		// fails we take down the service with it.
// 		task_manager
// 			.spawn_essential_handle()
// 			.spawn_blocking("aura", Some("block-authoring"), aura);
// 	}
//
// 	// if the node isn't actively participating in consensus then it doesn't
// 	// need a keystore, regardless of which protocol we use below.
// 	let keystore = if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };
//
// 	let grandpa_config = sc_finality_grandpa::Config {
// 		// FIXME #1578 make this available through chainspec
// 		gossip_duration: Duration::from_millis(333),
// 		justification_period: 512,
// 		name: Some(name),
// 		observer_enabled: false,
// 		keystore,
// 		local_role: role,
// 		telemetry: telemetry.as_ref().map(|x| x.handle()),
// 	};
//
// 	if enable_grandpa {
// 		// start the full GRANDPA voter
// 		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
// 		// this point the full voter should provide better guarantees of block
// 		// and vote data availability than the observer. The observer has not
// 		// been tested extensively yet and having most nodes in a network run it
// 		// could lead to finality stalls.
// 		let grandpa_config = sc_finality_grandpa::GrandpaParams {
// 			config: grandpa_config,
// 			link: grandpa_link,
// 			network,
// 			voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
// 			prometheus_registry,
// 			shared_voter_state: SharedVoterState::empty(),
// 			telemetry: telemetry.as_ref().map(|x| x.handle()),
// 		};
//
// 		// the GRANDPA voter task is considered infallible, i.e.
// 		// if it fails we take down the service with it.
// 		task_manager.spawn_essential_handle().spawn_blocking(
// 			"grandpa-voter",
// 			None,
// 			sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
// 		);
// 	}
//
// 	network_starter.start_network();
// 	Ok(task_manager)
// }
