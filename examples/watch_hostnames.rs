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
        let msg = match stream.next().await {
            Some(msg) => msg.unwrap(),
            None => continue,
        };

        // Join the hostnames into a single string.
        let mut hostnames = String::new();
        for hostname in msg.data.leaf_cert.all_domains {
            hostnames.push_str(&hostname);
            hostnames.push('|');
        }

        println!(
            "Source: {} | Hostname(s): {}",
            msg.data.source.name, hostnames
        );
    }
}
