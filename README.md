# certstream-rs
An unofficial port of [certstream-go](https://github.com/CaliDog/certstream-go) to Rust.


[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/certstream-rs.svg?maxAge=2592000)](https://crates.io/crates/certstream-rs)

This library streams Certificate Transparency Log notifications from https://certstream.calidog.io/ (or a custom instance of the [certstream-server](https://github.com/CaliDog/certstream-server)).

This was done as a small project to learn Rust. All comments/criticism/pull requests welcome -- the code quality is likely terrible.

# Example
```
> git clone https://github.com/Andoryuuta/certstream-rs
> cd certstream-rs
> cargo run --example watch_hostnames
```

# Usage
```rust
use certstreamrs::CertstreamClient;
use futures_util::{pin_mut, StreamExt};

#[tokio::main]
async fn main() {
    // Create the client and open the connection/stream.
    let client = CertstreamClient::default();
    let stream = client.watch_certs();
    pin_mut!(stream);

    // Read from the stream and print the certificate message.
    loop {
        let msg = stream.next().await.unwrap().unwrap();

        println!(
            "Source: {} | First Hostname: {}",
            msg.data.source.name,
            msg.data.leaf_cert.all_domains.first().unwrap()
        );
    }
}
```