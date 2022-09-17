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
    /// Host name (e.g. 'MyMachine')
    pub host: String,
    /// Service name (e.g. "_scanner")
    pub service: String,
    /// Protocol name (e.g. "_tcp")
    pub protocol: String,
    /// Port name (e.g. 53000)
    pub port: u16,
    /// TXT Records (in format of "key=value")
    pub txt_records: Vec<String>,
    /// Current State
    ///
    /// See [`ServiceState`]
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
    /// Name of the servide we are querying for
    pub name: String,
    /// Timeout until the next query is needed
    pub timeout: u64,
    /// Services resulting from the query
    /// When a service is completely resolved (IP and TXT records found)
    /// The service is returned as the next Stream item
    pub services: Vec<Service>,
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
    ///Prelude | State upon creation
    Prelude,
    /// WaitForFirstProbe | First random timeout sent
    WaitForFirstProbe,
    /// FirstProbe | First timeout finished    
    FirstProbe,
    /// WaitForSecondProbe | Query and second timeout sent   
    WaitForSecondProbe,
    /// SecondProbe | Second timeout finished  
    SecondProbe,
    /// WaitForAnnouncing | Probing finished waiting to be announced    
    WaitForAnnouncing,
    /// FirstAnnouncement | Ready to announce
    FirstAnnouncement,
    /// WaitForSecondAnnouncement | First announcement and timeout sent       
    WaitForSecondAnnouncement,
    /// SecondAnnouncement | Timeout finished, sending second announcement    
    SecondAnnouncement,
    /// Registered | Final state    
    Registered,
}

impl Default for ServiceState {
    fn default() -> Self {
        ServiceState::Prelude
    }
}
