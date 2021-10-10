## Ares cow 

### Start 

* With --request-base
```text
./target/release/node-template --tmp --dev --request-base http://141.164.58.241:5566
```

* Set ares author key by RPC request
```text
curl http://localhost:9933  -H "Content-Type:application/json;charset=utf-8" -d "@ocw-ares-01.curl"
```

* PRC data file is similar as below
```text
// ocw-ares-01.curl file content:
{
    "jsonrpc":"2.0",
    "id":1,
    "method":"author_insertKey",
    "params": [
        "ares",
        "XXXXX words ",
        "0xPublicKey of Hex"
    ]
}
```

### Testing

* Export the local chain spec to json
```text
./target/release/node-template build-spec --disable-default-bootnode --chain live > chain-data-ares-aura.json
```

* Start one
```text

./target/release/node-template purge-chain --base-path /tmp/aura/one --chain local -y
./target/release/node-template \
  --base-path /tmp/aura/one \
  --name ocw_one \
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
  --request-base http://141.164.58.241:5566 \
  --ares-keys-file /Users/mac/work-files/coding/git-files/ke-fan/ares-chain/ares_key_file_01.curl \
  --validator
  
```

* Start two
```text
./target/release/node-template purge-chain --base-path /tmp/aura/two --chain local -y
./target/release/node-template \
  --base-path /tmp/aura/two \
  --name ocw_two \
  --chain ./chain-data-ares-aura.json \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --ws-external \
  --rpc-external \
  --rpc-cors=all \
  --rpc-methods=Unsafe \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --request-base http://141.164.58.241:5566 \
  --ares-keys-file /Users/mac/work-files/coding/git-files/ke-fan/ares-chain/ares_key_file_02.curl \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
  
```

* Start tri
```text
./target/release/node-template purge-chain --base-path /tmp/aura/tri --chain local -y
./target/release/node-template \
  --base-path /tmp/aura/tri \
  --name ocw_tri \
  --chain ./chain-data-ares-aura.json \
  --port 30335 \
  --ws-port 9947 \
  --rpc-port 9935 \
  --ws-external \
  --rpc-external \
  --rpc-cors=all \
  --rpc-methods=Unsafe \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --request-base http://141.164.58.241:5566 \
  --ares-keys-file /Users/mac/work-files/coding/git-files/ke-fan/ares-chain/ares_key_file_03.curl \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
  
```

* Start four
```text
./target/release/node-template purge-chain --base-path /tmp/aura/four --chain local -y
./target/release/node-template \
  --base-path /tmp/aura/four \
  --name ocw_four \
  --chain ./chain-data-ares-aura.json \
  --port 30336 \
  --ws-port 9948 \
  --rpc-port 9936 \
  --ws-external \
  --rpc-external \
  --rpc-cors=all \
  --rpc-methods=Unsafe \
  --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' \
  --request-base http://141.164.58.241:5566 \
  --ares-keys-file /Users/mac/work-files/coding/git-files/ke-fan/ares-chain/ares_key_file_04.curl \
  --validator \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
  
```

* Add Aura keys
```text
curl http://localhost:9933  -H "Content-Type:application/json;charset=utf-8" -d "@ocw-aura-01.curl"
curl http://localhost:9934  -H "Content-Type:application/json;charset=utf-8" -d "@ocw-aura-02.curl"
curl http://localhost:9935  -H "Content-Type:application/json;charset=utf-8" -d "@ocw-aura-03.curl"
```

* Add GRANDPA key
```text
curl http://localhost:9933  -H "Content-Type:application/json;charset=utf-8" -d "@gran1.curl"
curl http://localhost:9934  -H "Content-Type:application/json;charset=utf-8" -d "@gran2.curl"
curl http://localhost:9935  -H "Content-Type:application/json;charset=utf-8" -d "@gran3.curl"

```

* Add ARES key
```text
curl http://localhost:9933  -H "Content-Type:application/json;charset=utf-8" -d "@ocw-ares-01.curl"
curl http://localhost:9934  -H "Content-Type:application/json;charset=utf-8" -d "@ocw-ares-02.curl"
curl http://localhost:9935  -H "Content-Type:application/json;charset=utf-8" -d "@ocw-ares-03.curl"
```