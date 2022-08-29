use std::time::Duration;

use dns_sd2::*;

#[tokio::main]
pub async fn main() {
    let mut client = DnsSd2::default();

    tokio::spawn(async {
        client.init().await;
    });

    client.register("_services._udp.local".into(), vec!["key=value".into()]);
}
