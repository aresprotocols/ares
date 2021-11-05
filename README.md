## Ares cow 

### Start with --dev

```text
./target/release/gladios-node --ws-port 9945 --tmp --dev --warehouse http://YourOracle:Port
```

### Start with network

#### Make ares key files
* Create a set of files to store private keys, such as `ares_key_files_**.txt`
* The content of the file is as follows, a
```text
aura:${Your_Mnemonic}
gran:${Your_Mnemonic}
```
* Aura uses sr25519, Gran uses ed25519.
* At least you need two sets of files. `ares_key_file_01.txt`, `ares_key_file_02.txt` are used in the example

### Start `bootnodes` validator

```text
./target/release/gladios-node purge-chain --base-path /tmp/aura/one --chain gladios -y
./target/release/gladios-node \
  --base-path /tmp/aura/one \
  --name ocw_one \
  --execution Native \
  --chain ./chain-data-ares-aura.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --ws-external \
  --rpc-external \
  --rpc-cors=all \
  --rpc-methods=Unsafe \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --warehouse http://YourOracle:Port \
  --ares-keys ./ares_key_file_01.txt \
  --validator
  
```

### Start `connection` validator
* Assume that the bootnode network is ws://127.0.0.1:9945
```text
./target/release/gladios-node purge-chain --base-path /tmp/aura/two --chain gladios -y
./target/release/gladios-node \
  --base-path /tmp/aura/two \
  --name ocw_two \
  --execution Native \
  --chain ./chain-data-ares-aura.json \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --ws-external \
  --rpc-external \
  --rpc-cors=all \
  --rpc-methods=Unsafe \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --warehouse http://YourOracle:Port \
  --ares-keys ./ares_key_file_02.txt \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

