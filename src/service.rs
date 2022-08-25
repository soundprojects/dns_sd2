use crate::ServiceState;

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
