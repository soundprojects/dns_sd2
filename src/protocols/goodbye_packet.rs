/// Send Goodbye Packets
///
/// Last step in MDNS shutdown protocol
///
/// When a service is dropped, send a goodbye record so other hosts know this service is gone
///
/// This step is only available if MdnsResolver state is `State::ShuttingDown`
///
/// [RFC6762 Section 10.1 - Goodbye Packets](https://www.rfc-editor.org/rfc/rfc6762#section-10.1)
/// - Send unsollicited response with a TTL of 0
pub async fn goodbye() -> (){
    todo!();
}