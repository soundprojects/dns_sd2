/// A Service is created by calling [`register()`]
///
/// Upon creation, the probing and announcing process is initiated by the
///
/// chain of handlers
///
/// Attribute | Value | Explanation
/// :-- |:-- |:--
/// Name | String | Service Name
/// Txt Records | Vec<String> | Txt Records in the format of `key=value`
/// State | [`ServiceState`] | State of the Service

#[derive(Debug, Default)]
pub struct Service {
    pub name: String,
    pub txt_records: Vec<String>,
    pub state: ServiceState,
}

/// A Query is created by calling [`browse()`]
///
/// Upon creation, the search process is initiated by the
///
/// chain of handlers
///
/// Attribute | Value | Explanation
/// :-- |:-- |:--
/// Name | String | Service Name
/// Timeout | u64 | Timeout until the next query
#[derive(Debug, Default)]
pub struct Query {
    _name: String,
    _timeout: u64,
}

/// Service State
///
/// Defines the state a [`Service`] is in during its lifetime
///
/// The state is either waiting for a timeout to finish
///
/// or for the chain to initiate the next action to make the service Registered

///  Value | Explanation
/// :-- |:--
///Prelude | State upon creation
/// WaitForFirstProbe | First random timeout sent
/// FirstProbe | First timeout finished
/// WaitForSecondProbe | Query and second timeout sent
/// SecondProbe | Second timeout finished
/// WaitForAnnouncing | Probing finished waiting to be announced
/// FirstAnnouncement | Ready to announce
/// WaitForSecondAnnouncement | First announcement and timeout sent
/// SecondAnnouncement | Timeout finished, sending second announcement
/// Registered | Final state
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ServiceState {
    Prelude,
    WaitForFirstProbe,
    FirstProbe,
    WaitForSecondProbe,
    SecondProbe,
    WaitForAnnouncing,
    FirstAnnouncement,
    WaitForSecondAnnouncement,
    SecondAnnouncement,
    Registered,
}

impl Default for ServiceState {
    fn default() -> Self {
        ServiceState::Prelude
    }
}
