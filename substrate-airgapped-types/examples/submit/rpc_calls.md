 # RPC calls to get data for creating a tx
 
 ```bash

 curl -X POST -H 'Content-Type: application/json' \
 -d '{"jsonrpc":"2.0","id": 1, "method":"state_getMetadata"}' \
 -o metadata.json http://localhost:9933

```

```bash

curl -X POST -H 'Content-Type: application/json' \
-d '{"jsonrpc":"2.0","id": 1, "method":"chain_getBlockHash", "params": [0]}' \
-o genesis.json http://localhost:9933

```