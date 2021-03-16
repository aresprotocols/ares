#!/usr/bin/env bash
set -ex

./target/debug/ares   --base-path /tmp/node   --chain res/testnet_raw.json  --port 30333   --ws-port 9944 --rpc-port 9933   --telemetry-url 'wss://telemetry.polkadot.io/submi 0'   --validator --rpc-methods Unsafe --name map002 --rpc-external --rpc-cors=all --ws-external --bootnodes /ip4/39.100.97.129/tcp/30333/p2p/12D3KooWKSegh2XE251GSvcajchc7QXyZot2xWzoN3gqzejeEF7q
