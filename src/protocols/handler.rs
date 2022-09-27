use crate::{
    message::MdnsMessage, record::ResourceRecord, service::ServiceState, MdnsError, Query, Service,
};

/// Chain of Responsibility Handler
///
/// The chain of responsibility pattern is used to loosely couple all the steps outlined in the Mdns and Dnssd RFC Specifications
///
/// When using the library as client you most often wish to browse or register a Service
///
/// You do this by calling init() on the Dns_Sd2 Struct which sets the event loop for this chain in motion
///
/// When calling browse() or query() an Event Enum is inserted into the chain along with the Records the struct has
///
/// Each part of the chain either does nothing with the Event or performs an action
///
/// For timing purposes like updating the Time To Live for records or for timeouts, such as in Probing or Announcing, a Event::TimeElepased event is sent into the chain
///
/// Each chain part implements the Handler trait

pub trait Handler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a>;
    fn handle(
        &self,
        event: &Event,
        records: &mut Vec<ResourceRecord>,
        registration: &mut Option<Service>,
        query: &mut Option<Query>,
        timeouts: &mut Vec<(ServiceState, u64)>,
        queue: &mut Vec<MdnsMessage>,
    ) -> Result<(), MdnsError>;
}

#[derive(Debug)]
/// Event Enumerator
///
/// Possible message types that are passed into the chain of handlers
/// They either pass elapsed times, close signals or messages that have arrived on the socket
pub enum Event {
    /// Message Enum containing an MdnsMessage
    Message(MdnsMessage),
    /// Time Elapsed, containing the Service State waiting for this timeout and the elapsed time
    TimeElapsed((ServiceState, u64)),
    /// TTL signal to update TTL (Each second)
    Ttl(),
    /// Close Signal
    Closing(),
    /// Browse Command, contains service string. e.g. '_myservice._udp._local'
    Browse(String),
    /// Register Command, contains
    Register(String, String, String, u16, Vec<String>),
}
