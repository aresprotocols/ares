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

## Ares-Gladios Builder Docker Image
### Build Image
```shell
docker build -t ares-chain -f docker/builder.Dockerfile  .
```

### Push Image to your repository
```shell
docker tag ares-chain:latest your-repository/image-name:tag
docker push your-repository/image-name:tag
```

### Run Image
```shell
docker run -d --name ares_gladios -p 9944:9944/tcp -v your-host-path:/data aresprotocollab/ares_gladios:beta gladios-node \
  --name your-name --chain gladios --ws-external --rpc-external \
  --rpc-cors=all --rpc-methods=Unsafe  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0'
```
> Note! 
> **your-host-path** must exist

