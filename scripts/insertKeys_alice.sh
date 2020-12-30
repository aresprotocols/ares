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
    "pair dismiss join hurdle eagle unit post general parent foam today game",
    "0xac4fe9958e3c225fbc2053239756308138d32e1fb983a98d4aa82aa68c097205"
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
    "pair dismiss join hurdle eagle unit post general parent foam today game",
    "0xdefe355beb2b22845ac48940dbf55d3dffe317e2491a57268e284a95902cc26c"
  ]
}
EOF