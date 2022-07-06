// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
	chain_spec,
	cli::{Cli, Subcommand},
	service,
};

use crate::service::Block;
use sc_cli::{ChainSpec, Result, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

pub const GLADIOS_RUNTIME_NOT_AVAILABLE: &str =
	"Gladios runtime is not available. Please compile the node with `--features with-gladios-runtime` to enable it.";
pub const PIONEER_RUNTIME_NOT_AVAILABLE: &str =
	"Pioneer runtime is not available. Please compile the node with `--features with-pioneer-runtime` to enable it.";
pub const ODYSSEY_RUNTIME_NOT_AVAILABLE: &str =
	"Odyssey runtime is not available. Please compile the node with `--features with-pioneer-runtime` to enable it.";


trait IdentifyChain {
	fn is_dev(&self) -> bool;
	fn is_gladios(&self) -> bool;
	fn is_pioneer(&self) -> bool;
	fn is_odyssey(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
	fn is_dev(&self) -> bool {
		self.id().starts_with("dev")
	}
	fn is_gladios(&self) -> bool {
		self.id().starts_with("gladios")
	}
	fn is_pioneer(&self) -> bool {
		self.id().starts_with("pioneer")
	}
	fn is_odyssey(&self) -> bool {
		self.id().starts_with("odyssey")
	}
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
	fn is_dev(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_dev(self)
	}
	fn is_gladios(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_gladios(self)
	}
	fn is_pioneer(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_pioneer(self)
	}
	fn is_odyssey(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_odyssey(self)
	}
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Ares Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			#[cfg(feature = "with-pioneer-runtime")]
			"dev" => {
				log::info!("🚅 🚅 🚅 load spec with development_config().");
				Box::new(chain_spec::pioneer::make_spec(
					self.spec_config.clone(),
					&include_bytes!("../res/dev.yml")[..],
				)?)
			},
			#[cfg(feature = "with-pioneer-runtime")]
			"test" => {
				log::info!("🚅 🚅 🚅 load spec with local_testnet_config().");
				Box::new(chain_spec::pioneer::make_spec(
					self.spec_config.clone(),
					&include_bytes!("../res/test.yml")[..],
				)?)
			},
			#[cfg(feature = "with-gladios-runtime")]
			"local" => {
				log::info!("🚅 🚅 🚅 load spec with local_testnet_config().");
				Box::new(chain_spec::gladios::make_spec(
					self.spec_config.clone(),
					&include_bytes!("../res/local.yml")[..],
				)?)
			},
			#[cfg(feature = "with-gladios-runtime")]
			"gladios" => {
				log::info!("🚅 🚅 🚅 load spec with local_gladios_config().");
				Box::new(chain_spec::gladios::make_spec(
					self.spec_config.clone(),
					&include_bytes!("../res/gladios.yml")[..],
				)?)
			},
			#[cfg(feature = "with-odyssey-runtime")]
			"" | "odyssey" | "live" => {
				log::info!("🚅 🚅 🚅 load spec with bytes.");
				if self.spec_config.is_some() {
					Box::new(chain_spec::odyssey::make_spec(
						self.spec_config.clone(),
						&include_bytes!("../res/odyssey.yml")[..],
					)?)
				} else {
					Box::new(chain_spec::odyssey::ChainSpec::from_json_bytes(
						&include_bytes!("../res/odyssey.json")[..],
					)?)
				}
			},
			path => {
				log::info!("🚅 🚅 🚅 load spec with json file.");
				let path = std::path::PathBuf::from(path);
				let chain_spec =
					Box::new(sc_service::GenericChainSpec::<(), chain_spec::Extensions>::from_json_file(path.clone())?)
						as Box<dyn ChainSpec>;
				// let chain_spec = chain_spec::pioneer::ChainSpec::from_json_file(std::path::PathBuf::from(path))?;
				if chain_spec.is_gladios() {
					#[cfg(feature = "with-gladios-runtime")]
					{
						Box::new(chain_spec::gladios::ChainSpec::from_json_file(path)?)
					}
					#[cfg(not(feature = "with-gladios-runtime"))]
					return Err(GLADIOS_RUNTIME_NOT_AVAILABLE.into())
				} else if chain_spec.is_odyssey() {
					#[cfg(feature = "with-odyssey-runtime")]
						{
							Box::new(chain_spec::odyssey::ChainSpec::from_json_file(path)?)
						}
					#[cfg(not(feature = "with-odyssey-runtime"))]
					return Err(ODYSSEY_RUNTIME_NOT_AVAILABLE.into())
				} else {
					#[cfg(feature = "with-pioneer-runtime")]
					{
						Box::new(chain_spec::pioneer::ChainSpec::from_json_file(path)?)
					}
					#[cfg(not(feature = "with-pioneer-runtime"))]
					return Err(PIONEER_RUNTIME_NOT_AVAILABLE.into())
				}
			},
		})
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		if chain_spec.is_gladios() {
			#[cfg(feature = "with-gladios-runtime")]
			return &gladios_runtime::VERSION;
			#[cfg(not(feature = "with-gladios-runtime"))]
			panic!("{}", GLADIOS_RUNTIME_NOT_AVAILABLE);
		} else if chain_spec.is_odyssey() {
			#[cfg(feature = "with-odyssey-runtime")]
			return &odyssey_runtime::VERSION;
			#[cfg(not(feature = "with-odyssey-runtime"))]
			panic!("{}", ODYSSEY_RUNTIME_NOT_AVAILABLE);
		} else {
			#[cfg(feature = "with-pioneer-runtime")]
			return &pioneer_runtime::VERSION;
			#[cfg(not(feature = "with-pioneer-runtime"))]
			panic!("{}", PIONEER_RUNTIME_NOT_AVAILABLE);
		}
	}
}

macro_rules! with_runtime_or_err {
	($chain_spec:expr, { $( $code:tt )* }) => {
		if $chain_spec.is_gladios() {
			#[cfg(feature = "with-gladios-runtime")]
			#[allow(unused_imports)]
			use service::gladios::{RuntimeApi,ExecutorDispatch};
			#[cfg(feature = "with-gladios-runtime")]
			$( $code )*

			#[cfg(not(feature = "with-gladios-runtime"))]
			return Err(GLADIOS_RUNTIME_NOT_AVAILABLE.into());
		} else if $chain_spec.is_odyssey() {
			#[cfg(feature = "with-odyssey-runtime")]
			#[allow(unused_imports)]
			use service::odyssey::{RuntimeApi,ExecutorDispatch};
			#[cfg(feature = "with-odyssey-runtime")]
			$( $code )*

			#[cfg(not(feature = "with-odyssey-runtime"))]
			return Err(GLADIOS_RUNTIME_NOT_AVAILABLE.into());
		} else {
			#[cfg(feature = "with-pioneer-runtime")]
			#[allow(unused_imports)]
			use service::pioneer::{RuntimeApi,ExecutorDispatch};
			#[cfg(feature = "with-pioneer-runtime")]
			$( $code )*

			#[cfg(not(feature = "with-pioneer-runtime"))]
			return Err(PIONEER_RUNTIME_NOT_AVAILABLE.into());
		}
	}
}

/// Parse and run command line arguments
pub fn run() -> Result<()> {
	let mut cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::Inspect(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			with_runtime_or_err!(chain_spec, {
				return runner.sync_run(|config| cmd.run::<Block, RuntimeApi, ExecutorDispatch>(config));
			})
		},
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			cli.spec_config = cmd.spec_config.clone();
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			with_runtime_or_err!(chain_spec, {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						service::new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				});
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			with_runtime_or_err!(chain_spec, {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						service::new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
					Ok((cmd.run(client, config.database), task_manager))
				});
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			with_runtime_or_err!(chain_spec, {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } =
						service::new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				});
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			with_runtime_or_err!(chain_spec, {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						service::new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				});
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			with_runtime_or_err!(chain_spec, {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						service::new_partial::<RuntimeApi, ExecutorDispatch>(&config)?;
					Ok((cmd.run(client, backend), task_manager))
				});
			})
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager = sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
					.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;

				Ok((cmd.run::<crate::service::Block, GExecutorDispatch>(config), task_manager))
			})
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::Benchmark(cmd)) =>
			if cfg!(feature = "runtime-benchmarks") {
				let runner = cli.create_runner(cmd)?;
				let chain_spec = &runner.config().chain_spec;
				with_runtime_or_err!(chain_spec, {
					return runner.sync_run(|config| cmd.run::<service::Block, ExecutorDispatch>(config));
				})
			} else {
				Err("Benchmarking wasn't enabled when building the node. You can enable it with \
				     `--features runtime-benchmarks`."
					.into())
			},
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				// ares params
				let mut ares_params: Vec<(&str, Option<Vec<u8>>)> = Vec::new();
				// if cli.run.validator || cli.run.shared_params.dev {
				if config.role.is_authority() {
					let request_base = match cli.warehouse {
						None => {
							panic!("⛔ Start parameter `--warehouse` is required!");
						},
						Some(request_url) => {
							if !request_url.starts_with("http://") &&
								!request_url.starts_with("https://") {
								panic!("⛔ `--warehouse` only supports http:// or https:// requests.");
							}
							request_url.as_str().as_bytes().to_vec()
						},
					};
					ares_params.push(("warehouse", Some(request_base)));

					match cli.ares_keys {
						None => {},
						Some(keys_file_path) => {
							ares_params.push(("ares-keys-file", Some(keys_file_path.as_bytes().to_vec())));
						},
					}
				}

				with_runtime_or_err!(config.chain_spec, {
					return service::new_full::<RuntimeApi, ExecutorDispatch>(config, ares_params)
						.map_err(sc_cli::Error::Service);
				})
			})
		},
	}
}
