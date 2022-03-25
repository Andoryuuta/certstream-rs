//! # certstream-rs
//! An unofficial port of [certstream-go](https://github.com/CaliDog/certstream-go) to Rust.
//!
//! This library streams Certificate Transparency Log notifications from <https://certstream.calidog.io/> (or a custom instance of the [certstream-server](https://github.com/CaliDog/certstream-server)).
//!
//! This was done as a small project to learn Rust. All comments/criticism/pull requests welcome -- the code quality is likely terrible.
//!
//! # Example
//! ```ignore
//! > git clone https://github.com/Andoryuuta/certstream-rs
//! > cd certstream-rs
//! > cargo run --example watch_hostnames
//! ```
//!
//! # Usage
//! ```rust
//! use certstreamrs::CertstreamClient;
//! use futures_util::{pin_mut, StreamExt};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create the client and open the connection/stream.
//!     let client = CertstreamClient::default();
//!     let stream = client.watch_certs();
//!     pin_mut!(stream);
//!
//!     // Read from the stream and print the certificate message.
//!     loop {
//!         let msg = stream.next().await.unwrap().unwrap();
//!
//!         println!(
//!             "Source: {} | First Hostname: {}",
//!             msg.data.source.name,
//!             msg.data.leaf_cert.all_domains.first().unwrap()
//!         );
//!     }
//! }
//! ```

use async_stream::try_stream;
use futures_util::StreamExt;
use std::error::Error;
use tokio_stream::Stream;

mod types;
use types::CertstreamMessage;

const DEFAULT_CERTSTREAM_WS_URL: &str = "wss://certstream.calidog.io";

pub struct CertstreamClient {
    url: String,
}

impl CertstreamClient {
    pub fn new(name: &str) -> Self {
        CertstreamClient {
            url: name.to_string(),
        }
    }

    pub fn watch_certs(&self) -> impl Stream<Item = Result<CertstreamMessage, Box<dyn Error>>> {
        let url = url::Url::parse(&self.url).unwrap();

        try_stream! {
            loop {
                // Connect to the certstream websocket server, which provides a constant stream of certificate notifications.
                // Bubble up error if this fails -- no easy recovery here.
                let (socket, _) = tokio_tungstenite::connect_async(&url).await?;
                let (_, mut read_socket) = socket.split();

                loop {
                    let msg_text = match read_socket.next().await {
                        Some(result) => {
                            match result {
                                Ok(msg) => match msg.into_text() {
                                    Ok(text) => text,
                                    Err(_) => {
                                        // Message couldn't be converted into text.
                                        continue;
                                    }
                                },
                                Err(_err) => {
                                    continue;
                                }
                            }
                        }
                        None => {
                            // End of websocket stream -- exit.
                            break;
                        }
                    };

                    let msg: types::CertstreamMessage = match serde_json::from_str(&msg_text) {
                        Ok(result) => result,
                        Err(err) => {
                            println!("Failed to parse json: {}", err);
                            println!("msg: {}", msg_text);
                            continue;
                        }
                    };

                    yield msg;
                }
            }
        }
    }
}

impl Default for CertstreamClient {
    fn default() -> Self {
        Self {
            url: DEFAULT_CERTSTREAM_WS_URL.to_string(),
        }
    }
}
