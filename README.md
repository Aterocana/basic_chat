# basic_chat
A little chat implemented while learning rust

## How to use it

First you need to start the server with `cargo run -bin server`, then you can connect with as many clients you need executing `cargo run --bin client`.

## Status

*TODO*

* It is still needed to handle client disconnections (now it tries to send messages also to disconnected clients);
* using `Message` struct in the client;
* try to reimplement it using some `Futures` library like `tokio`.
