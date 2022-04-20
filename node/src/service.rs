//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use sc_client_api::{Backend, BlockBackend, ExecutorProvider};
use sc_consensus_aura::SlotProportion;
pub use sc_executor::NativeElseWasmExecutor;
use sc_executor::NativeExecutionDispatch;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use seed_reader::{extract_content, make_author_insert_key_params, make_rpc_request};
use sp_api::ConstructRuntimeApi;
// use sp_consensus_aura::sr25519::{AuthorityId as AuraId, AuthorityPair as AuraPair};
use sp_core::{
	offchain::{OffchainStorage, STORAGE_PREFIX},
	Encode,
};
use sp_runtime::{
	generic, sp_std,
	traits::{BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
	MultiSignature,
};
use std::{io::Read, sync::Arc, time::Duration};
pub mod gladios;
pub mod pioneer;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;
pub type Balance = u128;
pub type BlockNumber = u32;

/// Index of a transaction in the chain. 32-bit should be plenty.
pub type Nonce = u32;
/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// use gladios_node::services;
/// Opaque, encoded, unchecked extrinsic.
pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

type FullClient<RuntimeApi, ExecutorDispatch> =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;

type FullBackend = sc_service::TFullBackend<Block>;

type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

type FullGrandpaBlockImport<RuntimeApi, ExecutorDispatch> = sc_finality_grandpa::GrandpaBlockImport<
	FullBackend,
	Block,
	FullClient<RuntimeApi, ExecutorDispatch>,
	FullSelectChain,
>;

/// A set of APIs that polkadot-like runtimes must implement.
pub trait RuntimeApiCollection:
	frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
	+ sp_api::ApiExt<Block>
	+ sp_consensus_babe::BabeApi<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ sp_api::Core<Block>
	+ sp_finality_grandpa::GrandpaApi<Block>
	+ sp_api::Metadata<Block>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
where
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

impl<Api> RuntimeApiCollection for Api
where
	Api: frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
		+ sp_api::ApiExt<Block>
		+ sp_consensus_babe::BabeApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ sp_api::Core<Block>
		+ sp_finality_grandpa::GrandpaApi<Block>
		+ sp_api::Metadata<Block>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>,
	<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
{
}

pub fn new_partial<RuntimeApi, ExecutorDispatch>(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient<RuntimeApi, ExecutorDispatch>,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient<RuntimeApi, ExecutorDispatch>>,
		sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, ExecutorDispatch>>,
		(
			(
				sc_consensus_babe::BabeBlockImport<
					Block,
					FullClient<RuntimeApi, ExecutorDispatch>,
					FullGrandpaBlockImport<RuntimeApi, ExecutorDispatch>,
				>,
				sc_finality_grandpa::LinkHalf<Block, FullClient<RuntimeApi, ExecutorDispatch>, FullSelectChain>,
				sc_consensus_babe::BabeLink<Block>,
			),
			Option<Telemetry>,
		),
	>,
	ServiceError,
>
where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	ExecutorDispatch: sc_executor::NativeExecutionDispatch + 'static,
{
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
		config.runtime_cache_size,
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

	let (block_import, babe_link) = sc_consensus_babe::block_import(
		sc_consensus_babe::Config::get(&*client)?,
		grandpa_block_import.clone(),
		client.clone(),
	)?;

	// let slot_duration = sc_consensus_aura::slot_duration(&*client)?.slot_duration();
	let slot_duration = babe_link.config().slot_duration();
	let import_queue = sc_consensus_babe::import_queue(
		babe_link.clone(),
		block_import.clone(),
		Some(Box::new(grandpa_block_import.clone())),
		client.clone(),
		select_chain.clone(),
		move |_, ()| async move {
			let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

			let slot = sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
				*timestamp,
				slot_duration,
			);

			let uncles = sp_authorship::InherentDataProvider::<<Block as BlockT>::Header>::check_inherents();

			Ok((timestamp, slot, uncles))
		},
		&task_manager.spawn_essential_handle(),
		config.prometheus_registry(),
		sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	// let grandpa_link_clone = grandpa_link.clone();
	let import_setup = (block_import, grandpa_link, babe_link);

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool,
		other: (import_setup, telemetry),
	})
}

/// Builds a new service for a full client.
pub fn new_full<RuntimeApi, ExecutorDispatch: NativeExecutionDispatch>(
	mut config: Configuration,
	ares_params: Vec<(&str, Option<Vec<u8>>)>,
) -> Result<TaskManager, ServiceError>
where
	RuntimeApi: ConstructRuntimeApi<Block, FullClient<RuntimeApi, ExecutorDispatch>> + Send + Sync + 'static,
	RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = sc_client_api::StateBackendFor<FullBackend, Block>>,
	ExecutorDispatch: sc_executor::NativeExecutionDispatch + 'static,
{
	let sc_service::PartialComponents::<
		FullClient<RuntimeApi, ExecutorDispatch>,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient<RuntimeApi, ExecutorDispatch>>,
		sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, ExecutorDispatch>>,
		(
			(
				sc_consensus_babe::BabeBlockImport<
					Block,
					FullClient<RuntimeApi, ExecutorDispatch>,
					FullGrandpaBlockImport<RuntimeApi, ExecutorDispatch>,
				>,
				sc_finality_grandpa::LinkHalf<Block, FullClient<RuntimeApi, ExecutorDispatch>, FullSelectChain>,
				sc_consensus_babe::BabeLink<Block>,
			),
			Option<Telemetry>,
		),
	> {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (import_setup, mut telemetry),
	} = new_partial(&config)?;

	//let auth_disc_publish_non_global_ips = config.network.allow_non_globals_in_dht;

	let grandpa_protocol_name = sc_finality_grandpa::protocol_standard_name(
		&client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
		&config.chain_spec,
	);
	let (block_import, grandpa_link, babe_link) = import_setup;

	config
		.network
		.extra_sets
		.push(sc_finality_grandpa::grandpa_peers_set_config(grandpa_protocol_name.clone()));
	let warp_sync = Arc::new(sc_finality_grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		grandpa_link.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, network_starter) = sc_service::build_network(sc_service::BuildNetworkParams {
		config: &config,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		spawn_handle: task_manager.spawn_handle(),
		import_queue,
		block_announce_validator_builder: None,
		warp_sync: Some(warp_sync),
	})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(&config, task_manager.spawn_handle(), client.clone(), network.clone());
	}

	let justification_stream = grandpa_link.justification_stream();
	let shared_authority_set = grandpa_link.shared_authority_set().clone();
	let shared_voter_state = sc_finality_grandpa::SharedVoterState::empty();
	let finality_proof_provider = sc_finality_grandpa::FinalityProofProvider::new_for_service(
		backend.clone(),
		Some(shared_authority_set.clone()),
	);

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks = Some(sc_consensus_slots::BackoffAuthoringOnFinalizedHeadLagging::default());
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();

	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();
		let backend = backend.clone();
		let shared_voter_state = shared_voter_state.clone();
		Box::new(move |deny_unsafe, subscription_executor| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				deny_unsafe,
				backend: backend.clone(),
				grandpa: crate::rpc::GrandpaDeps {
					shared_voter_state: shared_voter_state.clone(),
					shared_authority_set: shared_authority_set.clone(),
					justification_stream: justification_stream.clone(),
					subscription_executor,
					finality_provider: finality_proof_provider.clone(),
				},
			};

			Ok(crate::rpc::create_full(deps))
		})
	};

	let backend_clone = backend.clone();

	let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network: network.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		task_manager: &mut task_manager,
		transaction_pool: transaction_pool.clone(),
		rpc_extensions_builder,
		backend,
		system_rpc_tx,
		config,
		telemetry: telemetry.as_mut(),
	})?;

	log::info!("setting ares_params: {:?}", ares_params);
	let _result: Vec<(&str, bool)> = ares_params
		.iter()
		.map(|(order, x)| {
			match order {
				&"warehouse" => {
					match x {
						None => (*order, false),
						Some(exe_vecu8) => {
							let request_base_str = sp_std::str::from_utf8(exe_vecu8).unwrap();
							let store_request_u8 = request_base_str.encode();
							// let store_request_u8 = request_base_str.as_bytes();
							log::info!("setting request_domain: {:?}", request_base_str);
							if let Some(mut offchain_db) = backend_clone.offchain_storage() {
								log::debug!("after setting request_domain: {:?}", request_base_str);
								offchain_db.set(
									STORAGE_PREFIX,
									//LOCAL_STORAGE_PRICE_REQUEST_DOMAIN, // copy from ocw-suit
									b"are-ocw::price_request_domain",
									store_request_u8.as_slice(),
								);
							}
							(*order, true)
						},
					}
				},
				&"ares-keys-file" => {
					match x {
						None => (*order, false),
						Some(exe_vecu8) => {
							let key_file_path = sp_std::str::from_utf8(exe_vecu8).unwrap();
							let mut file = std::fs::File::open(key_file_path).unwrap();
							let mut contents = String::new();
							file.read_to_string(&mut contents).unwrap();
							let rawkey_list = extract_content(contents.as_str());
							let insert_key_list: Vec<(&str, &str, String)> =
								rawkey_list.iter().map(|x| make_author_insert_key_params(*x)).collect();
							let rpc_list: Vec<Option<String>> = insert_key_list
								.iter()
								.map(|x| make_rpc_request("author_insertKey", (x.0, x.1, x.2.as_str())))
								.collect();
							rpc_list.iter().any(|x| {
								if let Some(rpc_str) = x {
									// send rpc request.
									_rpc_handlers
										.io_handler()
										.handle_request_sync(rpc_str, sc_rpc::Metadata::default());
								}
								false
							});

							(*order, true)
						},
					}
				},
				&_ => ("NONE", false),
			}
		})
		.collect();

	if role.is_authority() {
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let can_author_with = sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

		let client_clone = client.clone();
		let slot_duration = babe_link.config().slot_duration();
		let babe_config = sc_consensus_babe::BabeParams {
			keystore: keystore_container.sync_keystore(),
			client: client.clone(),
			select_chain,
			env: proposer_factory,
			block_import,
			sync_oracle: network.clone(),
			justification_sync_link: network.clone(),
			create_inherent_data_providers: move |parent, ()| {
				let client_clone = client_clone.clone();
				async move {
					let uncles = sc_consensus_uncles::create_uncles_inherent_data_provider(&*client_clone, parent)?;

					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot = sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
						*timestamp,
						slot_duration,
					);

					let storage_proof =
						sp_transaction_storage_proof::registration::new_data_provider(&*client_clone, &parent)?;

					Ok((timestamp, slot, uncles, storage_proof))
				}
			},
			force_authoring,
			backoff_authoring_blocks,
			babe_link,
			can_author_with,
			block_proposal_slot_portion: SlotProportion::new(0.5),
			max_block_proposal_slot_portion: None,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		let babe = sc_consensus_babe::start_babe(babe_config)?;
		task_manager
			.spawn_essential_handle()
			.spawn_blocking("babe-proposer", Some("block-authoring"), babe);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore = if role.is_authority() { Some(keystore_container.sync_keystore()) } else { None };

	let grandpa_config = sc_finality_grandpa::Config {
		// FIXME #1578 make this available through chainspec
		gossip_duration: Duration::from_millis(333),
		justification_period: 512,
		name: Some(name),
		observer_enabled: false,
		keystore,
		local_role: role,
		telemetry: telemetry.as_ref().map(|x| x.handle()),
		protocol_name: grandpa_protocol_name,
	};

	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = sc_finality_grandpa::GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network,
			voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			sc_finality_grandpa::run_grandpa_voter(grandpa_config)?,
		);
	}

	// task_manager.spawn_essential_handle().spawn_blocking("ares-upgrade",
	// None,test_upgrade(client.clone()));

	network_starter.start_network();
	Ok(task_manager)
}

// pub fn test_upgrade<RuntimeApi, ExecutorDispatch>(
// 	client: Arc<FullClient<RuntimeApi, ExecutorDispatch>>,
// ) -> sp_blockchain::Result<impl Future<Output = ()> + Send> {
//     log::info!("{:?}",client.info().finalized_hash);
//     Ok(100)
// }
