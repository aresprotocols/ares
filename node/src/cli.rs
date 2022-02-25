use sc_cli::{GenericNumber, RunCmd};
use structopt::StructOpt;
use std::{fmt::Debug, str::FromStr, sync::Arc};
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};
use sc_client_api::{Backend, UsageProvider};
use sc_cli::{Result, CliConfiguration};
use sc_cli::{SharedParams, PruningParams};
use sp_runtime::traits::Zero;

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
	/// Run the revert command
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
