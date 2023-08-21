# the tmux usage
The configure file integration.yaml is used to start tango nodes and hardhat in tmux. Before run it, you should make sure all binaries are compiled, including the tango node, test node and contracts. And also install the tmux and tmuxp.

## install tmux 
```
brew install tmux
```

## install tmuxp
```
 brew install tmuxp
```

## install the mongo db locally
```
docker run --name mongodb -d -p 27017:27017 -v /tmp/mongo/data/db -e MONGO_INITDB_ROOT_USERNAME=tango -e MONGO_INITDB_ROOT_PASSWORD=tango mongo
```

## create both contracts and events collections.
```
use('tango_db');
db.createCollection('contracts')
db.createCollection('events')
```

## add the first contract of hardhat in mongo
```
db.contracts.insertOne({
  chain_endpoint: "ws://127.0.0.1:8545/",
  contract_address: "0x5FbDB2315678afecb367f032d93F642f64180aa3",
  event_type: "ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
})
```

## run tmux
```
tmuxp load integration.yaml
```

