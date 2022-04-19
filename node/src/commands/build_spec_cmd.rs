// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
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
use clap::Parser;
use sc_cli::{BuildSpecCmd, CliConfiguration, NodeKeyParams, SharedParams};
use sc_service::{config::NetworkConfiguration, ChainSpec};

/// The `build-spec` command used to build a specification.
#[derive(Debug, Clone, Parser)]
pub struct AresBuildSpecCmd {
	/// Name of file to spec configuration
	#[clap(long)]
	pub spec_config: Option<String>,

	#[clap(flatten)]
	build_spec_cmd: BuildSpecCmd,
}

impl AresBuildSpecCmd {
	/// Run the build-spec command
	pub fn run(&self, mut spec: Box<dyn ChainSpec>, network_config: NetworkConfiguration) -> sc_cli::Result<()> {
		self.build_spec_cmd.run(spec, network_config.clone())
	}
}

impl CliConfiguration for AresBuildSpecCmd {
	fn shared_params(&self) -> &SharedParams {
		&self.build_spec_cmd.shared_params
	}

	fn node_key_params(&self) -> Option<&NodeKeyParams> {
		Some(&self.build_spec_cmd.node_key_params)
	}
}
