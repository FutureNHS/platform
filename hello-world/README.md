# Hello World example

NOTE: This directory has been taken from the Medicines and Healthcare products Regulatory Agency (MHRA) 'Products' repository, which can be found [here](https://github.com/MHRA/products/tree/master/hello-world).

This service is a minimal example of an API that simply returns the string `Hello, SomeName` in response to an HTTP request to `/hello/SomeName`.

It is intentionally simple in order to aid learning, where everything that is not essential for a running service is left out. There are tests (because these are seen as essential 😇).

Please feel free to fork this repo and use this example as you wish (including submitting PRs).

## To get going

The API is built with [Rust](https://www.rust-lang.org/), using a web server crate called [tide](https://github.com/http-rs/tide), and the source code can be seen in [./src/main.rs](./src/main.rs).

Rust is used because it helps us write safe, reliable and fast services. It also compiles to a single binary and is small and lightweight - ideal for containers. This example uses Tide because it's a simple and idiomatic web server that is familiar and easy to understand and use.

1. Install the rust toolchain (see https://www.rust-lang.org/learn/get-started)
1. Run the tests to make sure everything is set up correctly: `cargo test`
1. Run locally: `cargo run`
1. Build a Docker image `make docker-build`
1. Run docker image `make docker-run`
1. Fork the repo, and submit a PR if you want to

## Linting

[rustfmt](https://github.com/rust-lang/rustfmt) is used for formatting our rust code.

## Benchmarks

1. Development build

   ```
   wrk -t4 -c100 -d30s http://127.0.0.1:3030/hello/me
   Running 30s test @ http://127.0.0.1:3030/hello/me
     4 threads and 100 connections
     Thread Stats   Avg      Stdev     Max   +/- Stdev
       Latency     2.77ms    2.21ms  25.48ms   81.43%
       Req/Sec    10.12k   737.16    12.10k    68.94%
     1212637 requests in 30.10s, 129.52MB read
   Requests/sec:  40285.45
   Transfer/sec:      4.30MB
   ```

2. Release build

   ```
   wrk -t4 -c100 -d30s http://127.0.0.1:3030/hello/me
   Running 30s test @ http://127.0.0.1:3030/hello/me
     4 threads and 100 connections
     Thread Stats   Avg      Stdev     Max   +/- Stdev
       Latency     0.88ms   79.78us   3.27ms   70.54%
       Req/Sec    28.22k     1.66k   33.58k    67.61%
     3381514 requests in 30.10s, 361.18MB read
   Requests/sec: 112340.84
   Transfer/sec:     12.00MB
   ```
