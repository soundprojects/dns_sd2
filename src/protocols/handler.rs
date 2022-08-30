use crate::{message::MdnsMessage, record::ResourceRecord, Query, Service};

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
        timeouts: &mut Vec<u64>,
    );
}

#[derive(Debug)]
pub enum Event {
    Message(MdnsMessage),
    TimeElapsed(u64),
    Closing(),
    Browse(String),
    Register(String, Vec<String>),
}
