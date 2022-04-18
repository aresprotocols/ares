use jsonrpc_core::serde_json;
use sc_cli::RunCmd;
use std::fmt::Debug;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[clap(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub run: RunCmd,

	// #[structopt(long = "warehouse")]
	// #[clap(warehouse)]
	#[clap(long)]
	pub warehouse: Option<String>,

	// #[structopt(long = "ares-keys")]
	// #[clap(ares-keys)]
	#[clap(long)]
	pub ares_keys: Option<String>,

	#[clap(long)]
	pub spec_config: Option<String>,
}

// #[derive(Debug, clap::Subcommand)]
// pub enum Subcommand {
// 	/// Key management cli utilities
// 	Key(sc_cli::KeySubcommand),
//
// 	/// Build a chain specification.
// 	BuildSpec(sc_cli::BuildSpecCmd),
//
// 	/// Validate blocks.
// 	CheckBlock(sc_cli::CheckBlockCmd),
//
// 	/// Export blocks.
// 	ExportBlocks(sc_cli::ExportBlocksCmd),
//
// 	/// Export the state of a given block into a chain spec.
// 	ExportState(sc_cli::ExportStateCmd),
//
// 	/// Import blocks.
// 	ImportBlocks(sc_cli::ImportBlocksCmd),
//
// 	/// Remove the whole chain.
// 	PurgeChain(sc_cli::PurgeChainCmd),
//
// 	/// Revert the chain to a previous state.
// 	Revert(sc_cli::RevertCmd),
//
// 	/// Revert the chain to a previous state.
// 	ForceRevert(ForceRevertCmd),
//
// 	/// The custom benchmark subcommmand benchmarking runtime pallets.
// 	#[structopt(name = "benchmark", about = "Benchmark runtime pallets.")]
// 	Benchmark(frame_benchmarking_cli::BenchmarkCmd),
//
// 	/// Get grandpa current authorities set
// 	GrandpaState(GrandpaStateCmd),
// }

/// Possible subcommands of the main binary.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// The custom inspect subcommmand for decoding blocks and extrinsics.
	#[clap(name = "inspect", about = "Decode given block or extrinsic using current native runtime.")]
	Inspect(node_inspect::cli::InspectCmd),

	/// The custom benchmark subcommmand benchmarking runtime pallets.
	#[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,

	/// Key management cli utilities
	#[clap(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Verify a signature for a message, provided on STDIN, with a given (public or secret) key.
	Verify(sc_cli::VerifyCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(sc_cli::VanityCmd),

	/// Sign a message, with a given (secret) key.
	Sign(sc_cli::SignCmd),

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
	/* /// Revert the chain to a previous state.
	 * ForceRevert(ForceRevertCmd), */

	/* 	/// Get grandpa current authorities set
	 * 	GrandpaState(GrandpaStateCmd), */
}

// #[derive(Debug, StructOpt)]
// pub struct ForceRevertCmd {
// 	/// Number of blocks to revert.
// 	#[clap(default_value = "256")]
// 	pub num: GenericNumber,
//
// 	#[allow(missing_docs)]
// 	#[clap(flatten)]
// 	pub shared_params: SharedParams,
//
// 	#[allow(missing_docs)]
// 	#[clap(flatten)]
// 	pub pruning_params: PruningParams,
// }
//
// impl ForceRevertCmd {
// 	pub async fn run<B, BA, C>(&self, client: Arc<C>, backend: Arc<BA>) -> Result<()>
// 	where
// 		B: BlockT,
// 		BA: Backend<B>,
// 		C: UsageProvider<B>,
// 		<<<B as BlockT>::Header as HeaderT>::Number as FromStr>::Err: Debug,
// 	{
// 		let blocks = self.num.parse()?;
// 		// let force = self.force.parse().unwrap_or(false);
// 		// revert_chain(client, backend, blocks)?;
// 		let reverted = backend.revert(blocks, true)?;
// 		let info = client.usage_info().chain;
//
// 		if reverted.0.is_zero() {
// 			log::info!("There aren't any non-finalized blocks to force revert.");
// 		} else {
// 			log::info!("Force reverted {} blocks. Best: #{} ({})", reverted.0, info.best_number,
// info.best_hash); 		}
// 		Ok(())
// 	}
// }
//
// impl CliConfiguration for ForceRevertCmd {
// 	fn shared_params(&self) -> &SharedParams {
// 		&self.shared_params
// 	}
//
// 	fn pruning_params(&self) -> Option<&PruningParams> {
// 		Some(&self.pruning_params)
// 	}
// }

// #[derive(Debug, StructOpt)]
// pub struct GrandpaStateCmd {
// 	#[allow(missing_docs)]
// 	#[structopt(flatten)]
// 	pub shared_params: SharedParams,
//
// 	#[allow(missing_docs)]
// 	#[structopt(flatten)]
// 	pub pruning_params: PruningParams,
// }
//
// impl CliConfiguration for GrandpaStateCmd {
// 	fn shared_params(&self) -> &SharedParams {
// 		&self.shared_params
// 	}
//
// 	fn pruning_params(&self) -> Option<&PruningParams> {
// 		Some(&self.pruning_params)
// 	}
// }
//
// impl GrandpaStateCmd {
// 	//LinkHalf<Block: BlockT, C, SC>
// 	pub async fn run<Block, C, SC>(&self, link_half: sc_finality_grandpa::LinkHalf<Block, C, SC>) ->
// Result<()> 	where
// 		Block: BlockT,
// 		C: UsageProvider<Block>,
// 	{
// 		use sp_serializer as ser;
// 		let authority_set = link_half.shared_authority_set();
// 		let authority_set = authority_set.clone_inner();
// 		log::info!("{}", ser::to_string_pretty(&authority_set));
// 		Ok(())
// 	}
// }
