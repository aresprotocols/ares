use jsonrpc_core::serde_json;
use sc_cli::{CliConfiguration, GenericNumber, PruningParams, Result, RunCmd, SharedParams};
use sc_client_api::{Backend, UsageProvider};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT, Zero};
use std::{fmt::Debug, str::FromStr, sync::Arc};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,

	#[structopt(long = "warehouse")]
	pub warehouse: Option<String>,

	#[structopt(long = "ares-keys")]
	pub ares_keys: Option<String>,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
	/// Key management cli utilities
	Key(sc_cli::KeySubcommand),
	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Revert the chain to a previous state.
	ForceRevert(ForceRevertCmd),

	/// The custom benchmark subcommmand benchmarking runtime pallets.
	#[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Get grandpa current authorities set
	GrandpaState(GrandpaStateCmd),
}

/// The `revert` command used revert the chain to a previous state.
#[derive(Debug, StructOpt)]
pub struct ForceRevertCmd {
	/// Number of blocks to revert.
	#[structopt(default_value = "256")]
	pub num: GenericNumber,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub pruning_params: PruningParams,
}

impl ForceRevertCmd {
	pub async fn run<B, BA, C>(&self, client: Arc<C>, backend: Arc<BA>) -> Result<()>
	where
		B: BlockT,
		BA: Backend<B>,
		C: UsageProvider<B>,
		<<<B as BlockT>::Header as HeaderT>::Number as FromStr>::Err: Debug,
	{
		let blocks = self.num.parse()?;
		// let force = self.force.parse().unwrap_or(false);
		// revert_chain(client, backend, blocks)?;
		let reverted = backend.revert(blocks, true)?;
		let info = client.usage_info().chain;

		if reverted.0.is_zero() {
			log::info!("There aren't any non-finalized blocks to force revert.");
		} else {
			log::info!("Force reverted {} blocks. Best: #{} ({})", reverted.0, info.best_number, info.best_hash);
		}
		Ok(())
	}
}

impl CliConfiguration for ForceRevertCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn pruning_params(&self) -> Option<&PruningParams> {
		Some(&self.pruning_params)
	}
}

#[derive(Debug, StructOpt)]
pub struct GrandpaStateCmd {
	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub shared_params: SharedParams,

	#[allow(missing_docs)]
	#[structopt(flatten)]
	pub pruning_params: PruningParams,
}

impl CliConfiguration for GrandpaStateCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.shared_params
	}

	fn pruning_params(&self) -> Option<&PruningParams> {
		Some(&self.pruning_params)
	}
}

impl GrandpaStateCmd {
	//LinkHalf<Block: BlockT, C, SC>
	pub async fn run<Block, C, SC>(&self, link_half: sc_finality_grandpa::LinkHalf<Block, C, SC>) -> Result<()>
	where
		Block: BlockT,
		C: UsageProvider<Block>,
	{
		use sp_serializer as ser;
		let authority_set = link_half.shared_authority_set();
		let authority_set = authority_set.clone_inner();
		log::info!("{}", ser::to_string_pretty(&authority_set));
		Ok(())
	}
}