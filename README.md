# tss-tango
A app that utilizes frost-dalek to verify messages sent between siloed networks

## Building
To run this build make sure you have protobuff installed.
For mac use: <br />
```Bash
brew install protobuf
```

For linux use: <br />
```Bash
sudo apt-get install protobuf-compiler
```

## Benchmarking

Warning this program can kill all your tmux sessions

### Requirements:
* Make sure you have mongodb running with a database called `tango_db` and a collection called `events`
* Make sure you have [tmux](https://github.com/tmux/tmux/wiki/Installing) installed on your system for more info read scripts/tmux/readme.md

* Make build of example file

```Bash
cargo build --release --example tss_bench_run
```

* Run the benchmarking script

```Bash
cargo run --example tss_bench_n3t2_1000
```

### phase one
based on clarified requirements, make a runnable system. the design as [diagram](https://www.figma.com/file/IGAVjmZfMwJqVt0XBiy9es/Untitled?node-id=57%3A286)

development plan:
- [] connector can get events, stored in db with single signature. test passed with local ethereum
- [] tss works, tango node can exchange key and sign event
- [] mutliple signature with event. test passed with ethereum test net
- [] ci works, integration test automatically run. node number and threshold configurable (3 nodes to 10 nodes)


## How to run the nodes

* Make sure you have hardhat setup

Try running some of the following tasks:

```shell
npx hardhat node
npx hardhat compile
npx hardhat run scripts/deploy.ts --network localhost
npx hardhat run scripts/getAmount.ts  --network localhost
```

* Navigate to the root folder and start your nodes

```shell
target/release/tango-node --db-url mongodb://localhost:27017/admin --blockchain=ethereum
```

open another terminal

```shell
target/release/tango-node --new-node=true
```

Finally run the last node

```shell
target/release/tango-node --new-node=true
```