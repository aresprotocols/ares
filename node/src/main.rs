//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod cli;
mod command;
mod rpc;
mod services;
mod grandpa_hard_forks;

fn main() -> sc_cli::Result<()> {
	command::run()
}
