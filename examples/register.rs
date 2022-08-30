use dns_sd2::*;
use futures::{pin_mut, StreamExt};
use log::debug;

#[tokio::main]
pub async fn main() {
    let mut client = DnsSd2::default();

    let stream = client
        .register("_services._udp.local".into(), vec!["key=value".into()])
        .await;

    pin_mut!(stream);

    while let Some(Ok(s)) = stream.next().await {
        debug!("GOT A SERVICE");
    }
}
