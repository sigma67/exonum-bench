# exonum-timestamping benchmark
Simple client-based benchmarking tool for the [exonum-timestamping service](https://github.com/exonum/exonum/tree/master/examples/timestamping).

This tool was built from the need to have a **high-throughput and efficient benchmarking tool** for the Exonum blockchain. Using the nodeJS client for Exonum resulted in inconsistent results and would often be bottlenecked by the sending client.

By reusing the [transactions schema](https://github.com/sigma67/exonum-bench/tree/master/src/transactions.rs) from the Rust service implementation it becomes possible to properly benchmark the blockchain. 

An adjusted frontend for the timestamping service with deployment and benchmarking scripts can be found at https://bitbucket.org/sigma67/exonum-log-timestamping/

## Research Paper
You can find the associated research paper here: 

[A secure and auditable logging infrastructure based on a permissioned blockchain](https://www.sciencedirect.com/science/article/pii/S0167404818313907)

## Prerequisites
Initially you need to set up at least 4 instances of the Exonum blockchain with the timestamping service by following the instructions in examples/timestamping/README.md
```sh
# clone the main Exonum repository v0.11
git clone https://github.com/exonum/exonum/ --branch "0-11-1-release" --depth 1

cd exonum/examples/timestamping/backend
cargo install
```

To install the frontend, run 
```sh
git clone https://bitbucket.org/sigma67/exonum-log-timestamping/ --depth 1
npm install
npm run build
npm start -- --port=2268 --api-root=http://127.0.0.1:8200
```

## Build the benchmarking tool
```sh
# Clone this repository
git clone https://github.com/sigma67/exonum-bench
cargo install
```
For the build to succeed, the main exonum repository needs to be present in the same parent directory.
## Run
The benchmarking tool sends a specific number of transactions to the configured blockchain node per second.
```
# Send 100 transactions per second for 60 seconds
exonum-bench 100 60 1
```

There is also an automated script for remote benchmarking spread across all blockchain nodes at https://bitbucket.org/sigma67/exonum-log-timestamping/src/master/deploy/run_benchmark.sh.

This results in a more realistic benchmark since all blockchain nodes receive an equal amount of transactions.
