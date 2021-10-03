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