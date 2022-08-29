use crate::{
    create_socket,
    message::MdnsMessage,
    question::{QClass, QType, Question},
    record::ResourceRecord,
    ServiceState,
};
use std::io;
use tokio_util::udp::UdpFramed;

/// MDNS Service Struct
///
/// MdnsService is a struct that is initialized by the MdnsController upon use when calling `browse()` or `register()`
///
/// The structure holds all the necessary information for querying and responding for the service
///
/// ## Example
///
pub struct MdnsService {
    pub state: ServiceState,
}

impl MdnsService {
    pub fn start() {}

    pub fn drop() {}
}

pub struct MdnsBrowse {
    pub name: String,
    pub records: Vec<ResourceRecord>,
}

impl MdnsBrowse {
    pub fn search(name: &str) {
        // Start loop
        // Select with query interval
        // Update TTL every second
        // Channel for quitting
        // Listen for responses
        //We found something!
    }
    /// Update TTL
    ///
    /// Update TTL Values for the given records
    ///
    ///
    /// [RFC6762 Section 10 - Resource Record TTL Values and Cache Coherency](https://www.rfc-editor.org/rfc/rfc6762#section-10)
    ///
    /// Most DNS TTL are set to a 75 minute default
    /// Other responses where the host name is equal to the record name (A, AAAA, SRV) are set to 120 seconds
    /// When the TTL default is down by 80%, a new query is necessary and 85, 90 and 95%.
    /// There should be a 2% offset of the TTL query to prevent simultaneous queries by multiple systems
    ///
    /// Only records that are of an active interest to a local client are in need of this cache maintenance
    /// [RFC6762 Section 5.2 - Continuous Multicast DNS Querying](https://www.rfc-editor.org/rfc/rfc6762#section-5.2)
    ///
    /// - Decrease TTL for each record by 1
    /// - Verify if TTL cache rules are met
    /// - Notify if new query is necessary
    pub fn update_ttl(&mut self) -> io::Result<()> {
        todo!();
    }

    /// Query
    ///
    /// Sends a query for the given browse
    /// First series of queries should be delayed by 20-120 ms
    ///
    /// The time between multiple queries starts at 1s and is doubled each time. A maximum cap of 60 minutes may be set.
    ///
    /// [RFC6762 Section 5.2 - Continuous Multicast DNS Querying](https://www.rfc-editor.org/rfc/rfc6762#section-5.2)
    pub async fn query(&mut self) {
        let udp_socket = create_socket().expect("Failed to create Socket");

        let mut message = MdnsMessage::default();

        message.header.qdcount = 1;

        message.questions.push(Question {
            name: "_special._udp.local".to_string(),
            qtype: QType::Any,
            qclass: QClass::Any,
        });

        udp_socket
            .send_to(&message.to_bytes(), "224.0.0.51:5353")
            .await
            .expect("Failed to send Mdns Message");

        let udp_framed = UdpFramed::new();
    }
}

#[tokio::test]
pub async fn test_query() {
    let browse = MdnsBrowse::search("_special._udp.local");
}
