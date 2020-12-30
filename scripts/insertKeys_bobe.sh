#!/usr/bin/env zsh
set -ex

# Alice keys for Aura
curl http://127.0.0.1:9933 -H "Content-Type:application/json;charset=utf-8" \
--data-binary @- << EOF
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "aura",
    "advance gorilla prepare search belt devote vocal envelope page smooth adjust core",
    "0xe0dcf11f6d46ffc50892ea238900b4f94dbd7a201de141f2697dff5671739654"
  ]
}
EOF

# Alice's keys for GRANDPA
curl http://127.0.0.1:9933 -H "Content-Type:application/json;charset=utf-8" \
--data-binary @- << EOF
{
  "jsonrpc":"2.0",
  "id":1,
  "method":"author_insertKey",
  "params": [
    "gran",
    "advance gorilla prepare search belt devote vocal envelope page smooth adjust core",
    "0xbb77ca0202b7864f39354d36d282ebf0e5f71490f767867a889f790ddc5b0067"
  ]
}
EOF