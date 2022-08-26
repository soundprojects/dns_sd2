use crate::{record::ResourceRecord, ServiceState};
use std::io;

/// MDNS Service Struct
///
/// MdnsService is a struct that is initialized by the MdnsController upon use when calling `browse()` or `register()`
///
/// The structure holds all the necessary information for querying and responding for the service
pub struct MdnsService {
    pub state: ServiceState,
}

impl MdnsService {
    pub fn start() {}

    pub fn drop() {}
}

pub struct MdnsBrowse {
    pub records: Vec<ResourceRecord>,
}

impl MdnsBrowse {
    pub fn search(on_result: Box<dyn FnOnce() + Send>) {
        // Start loop
        // Select with query interval
        // Update TTL every second
        // Channel for quitting
        // Listen for responses
        //We found something!
        on_result();
    }
    /// Update TTL
    ///
    /// Update TTL Values for the given records
    ///
    ///
    /// [RFC1035 Section 10 - Resource Record TTL Values and Cache Coherency](https://www.rfc-editor.org/rfc/rfc6762#section-10)
    ///
    /// Most DNS TTL are set to a 75 minute default
    /// Other responses where the host name is equal to the record name (A, AAAA, SRV) are set to 120 seconds
    /// When the TTL default is down by 80%, a new query is necessary
    ///
    /// - Decrease TTL for each record by 1
    /// - Verify if TTL cache rules are met
    /// - Notify if new query is necessary
    pub fn update_ttl(&mut self) -> io::Result<()> {
        todo!();
    }

    pub fn query(&mut self) {}
}
