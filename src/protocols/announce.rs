
/// Announce MDNS Service
///
/// Second step in MDNS announcement protocol
///
/// This step is only available if MdnsResolver state is `State::Announcing`
///
/// [RFC6762 Section 8.3 - Announcing](https://www.rfc-editor.org/rfc/rfc6762#section-8.3)
/// - Send unsollicited response with all answers, both shared and unique
/// - For the unique records, set cache flush bit to '1'
/// - Wait 1s
/// - Send unsollicited response again
pub async fn announce() -> io::Result<()> {
    let random_delay = rand::thread_rng().gen_range(0..250);
    tokio::time::sleep(std::time::Duration::from_millis(random_delay)).await;

    //TODO Send unsollicited response

    //TODO Select statement with receiving and parsing / timer
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    //TODO Send unsollicited response

    Ok(())
}