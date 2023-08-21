# Network (In Progress)
P2P connection between tesseract nodes, based on libp2p package.

## Testing network communication
Make sure you are in network directory `cd network`<br />
You can run test using `cargo test` <br />
Or use the command `cargo test -- --nocapture` to see detailed output <br />
Or you can run tests one by one using following commands<br />
`cargo test -- tests::communication_with_seed --exact --nocapture` <br />
`cargo test -- tests::communication_with_mdns --exact --nocapture` <br />

## Running a node
Make sure you are in Tesseract folder.<br />

Running a node with default properties use:<br />
`cargo run`<br />
Running node on specific port using<br />
`cargo run -- --node-port <Port>`<br />
Running node on specific topic<br />
`cargo run -- --topic <Topic Name>`<br />
Running node with bootstrap node<br />
`cargo run -- --seed-node <Node Address>`<br />
Running node with expicit peer to gossip protocol<br />
`cargo run -- --explicit-peer <Node Address>`<br />
Running node with new identity<br />
`cargo run -- --new-node <Boolean(true/false)>`<br />

## Running multiple nodes on same Computer / LAN
Multiple nodes can be run from same computer or different computer but same internet connection.
To start multiple nodes on same computer run `cargo run -- --new-node=true` command from each termina or you can use `cargo run` if you are on different computers. Make sure to change message sent from gossip protocol before running other node:<br /> 
go to<br />

```
tesseract
|__src
|____main.rs <-
```

`let _ = event_sender.send("<new message here>".into()).await;`<br />
so that both nodes send each other a different message.


##  development plan:
- [] nodes can find each other via seed node
- [] event signature send/receive works
- [] event finalized send/receiver works
- [] configure change send/receiver works