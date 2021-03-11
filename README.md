#ares: Completely Decentralized Oracle Protocol

[![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](LICENSE) [![GitLab Status](https://gitlab.parity.io/parity/substrate/badges/master/pipeline.svg)](https://gitlab.parity.io/parity/substrate/pipelines) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.adoc)

Ares is a predictive machine project based on Substrate, with the objective of providing safe and credible under chain real data use a decentralized approach for smart contracts, parallel chains or other projects in the ecosystem of the Polkadot.

It is a decentralized oracle network that consists of Ares oracle Module, it makes full use of the off-chain worker, sources aggregator committee random mine block and reputation council.

## Note

Now we are mainly testing the functions of block generation, transfer, staking, etc. We will open more codes later when we are ready.

## Running from Source

### Building
Install all the required dependencies with a single command (be patient, this can take up to 30 minutes).

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
```

Once the development environment is set up, build the node.

```bash
git clone https://github.com/aresprotocols/ares.git
cd ares
make init
make build
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/ares -h
```

