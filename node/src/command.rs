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
	services,
};
// use runtime_gladios_node::Block as GladiosNodeBlock;
// use runtime_pioneer_node::Block as PioneerNodeBlock;

use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

trait IdentifyChain {
	fn is_dev(&self) -> bool;
	fn is_gladios(&self) -> bool;
	fn is_pioneer(&self) -> bool;
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
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Substrate Node".into()
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

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => {
				log::info!("🚅 🚅 🚅 load spec with development_config().");
				Box::new(chain_spec::development_config()?)
			},
			"local" => {
				log::info!("🚅 🚅 🚅 load spec with local_testnet_config().");
				Box::new(chain_spec::local_ares_config()?)
			},
			"test" => {
				log::info!("🚅 🚅 🚅 load spec with local_testnet_config().");
				Box::new(chain_spec::local_testnet_config()?)
			},
			"" | "gladios" | "live" => {
				log::info!("🚅 🚅 🚅 load spec with bytes.");
				Box::new(chain_spec::GladiosNodeChainSpec::from_json_bytes(
					&include_bytes!("../res/chain-data-ares-aura-raw.json")[..],
				)?)
			},
			path => {
				log::info!("🚅 🚅 🚅 load spec with json file.");
				// Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?)
				let chain_spec = chain_spec::PioneerNodeChainSpec::from_json_file(std::path::PathBuf::from(path))?;
				if chain_spec.is_gladios() {
					Box::new(chain_spec::GladiosNodeChainSpec::from_json_file(std::path::PathBuf::from(path))?)
				} else {
					Box::new(chain_spec)
				}
			},
		})
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		if chain_spec.is_gladios() {
			&runtime_gladios_node::VERSION
		} else {
			&runtime_pioneer_node::VERSION
		}
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();
	use services::{gladios::ExecutorDispatch as GExecutorDispatch, pioneer::ExecutorDispatch as PExecutorDispatch};
	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			if runner.config().chain_spec.is_pioneer() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						services::pioneer::new_partial(&config)?;
					// services::<runtime_pioneer_node::RuntimeApi, PExecutorDispatch>::new_partial(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					services::gladios::new_partial(&config)?;
				// services::<runtime_gladios_node::RuntimeApi, GExecutorDispatch>::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			if runner.config().chain_spec.is_pioneer() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } = services::pioneer::new_partial(&config)?;
					// services::<runtime_pioneer_node::RuntimeApi, PExecutorDispatch>::new_partial(&config)?;

					Ok((cmd.run(client, config.database), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = services::gladios::new_partial(&config)?;
				// services::<runtime_gladios_node::RuntimeApi, GExecutorDispatch>::new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			if runner.config().chain_spec.is_pioneer() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, .. } = services::pioneer::new_partial(&config)?;
					// services::<runtime_pioneer_node::RuntimeApi, PExecutorDispatch>::new_partial(&config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = services::gladios::new_partial(&config)?;
				//services::<runtime_gladios_node::RuntimeApi, GExecutorDispatch>::new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			if runner.config().chain_spec.is_pioneer() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, import_queue, .. } =
						services::pioneer::new_partial(&config)?;
					//services::<runtime_pioneer_node::RuntimeApi, PExecutorDispatch>::new_partial(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					services::gladios::new_partial(&config)?;
				// services::<runtime_gladios_node::RuntimeApi, GExecutorDispatch>::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			if runner.config().chain_spec.is_pioneer() {
				return runner.async_run(|config| {
					let PartialComponents { client, task_manager, backend, .. } =
						services::pioneer::new_partial(&config)?;
					// services::<runtime_pioneer_node::RuntimeApi, PExecutorDispatch>::new_partial(&config)?;
					Ok((cmd.run(client, backend), task_manager))
				})
			}
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } = services::gladios::new_partial(&config)?;
				// services::<runtime_gladios_node::RuntimeApi, GExecutorDispatch>::new_partial(&config)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		},
		Some(Subcommand::Benchmark(cmd)) =>
			if cfg!(feature = "runtime-benchmarks") {
				let runner = cli.create_runner(cmd)?;
				if runner.config().chain_spec.is_pioneer() {
					runner.sync_run(|config| cmd.run::<services::Block, PExecutorDispatch>(config))
				} else {
					runner.sync_run(|config| cmd.run::<services::Block, GExecutorDispatch>(config))
				}
			} else {
				Err("Benchmarking wasn't enabled when building the node. You can enable it with \
				     `--features runtime-benchmarks`."
					.into())
			},
		None => {
			let runner = cli.create_runner(&cli.run)?;
			let is_pioneer = runner.config().chain_spec.is_pioneer();
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
							if !request_url.starts_with("http") {
								panic!("⛔ `--warehouse` only supports http requests.");
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

				if is_pioneer {
					services::pioneer::new_full(config, ares_params).map_err(sc_cli::Error::Service)
				// services::<runtime_pioneer_node::RuntimeApi, PExecutorDispatch>::new_full(config,
				// ares_params) 	.map_err(sc_cli::Error::Service)
				} else {
					services::gladios::new_full(config, ares_params).map_err(sc_cli::Error::Service)
					// services::<runtime_gladios_node::RuntimeApi,
					// GExecutorDispatch>::new_full(config, ares_params) 	.map_err(sc_cli::Error::
					// Service)
				}
			})
		},
	}
}
