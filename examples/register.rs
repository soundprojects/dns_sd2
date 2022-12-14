use dns_sd2::*;
use futures::{pin_mut, StreamExt};
use log::debug;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init_timed();

    let mut client = DnsSd2::default();

    let stream = client
        .register(
            "MyMac".into(),
            "_special".into(),
            "_tcp".into(),
            53000,
            vec!["key=value".into()],
        )
        .await;

    pin_mut!(stream);

    while let Some(result) = stream.next().await {
        match result {
            Ok(service) => {
                debug!("Got OK {:?}", service)
            }
            Err(e) => {
                debug!("Got Error {}", e)
            }
        }
    }
}
