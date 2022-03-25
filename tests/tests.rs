#[cfg(test)]
mod tests {
    use certstreamrs::CertstreamClient;
    use futures_util::{pin_mut, StreamExt};

    #[tokio::test]
    async fn subscribe_test() {
        let client = CertstreamClient::default();
        let stream = client.watch_certs();
        pin_mut!(stream);

        let msg = stream.next().await;
        assert!(msg.is_some());
    }
}
