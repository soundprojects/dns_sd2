use crate::{message::MdnsMessage, record::ResourceRecord, service::ServiceState, Query, Service};

use super::handler::{Event, Handler};

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
#[derive(Default, Copy, Clone)]
pub struct GoodbyeHandler<'a> {
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> Handler<'a> for GoodbyeHandler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
    fn handle(
        &self,
        event: &Event,
        records: &mut Vec<ResourceRecord>,
        registration: &mut Option<Service>,
        query: &mut Option<Query>,
        timeouts: &mut Vec<(ServiceState, u64)>,
        queue: &mut Vec<MdnsMessage>,
    ) {
        if let Some(r) = registration {
            match event {
                Event::Closing() => {
                    info!("Sending Goodbye Packets!");
                    queue.push(MdnsMessage::goodbye(&r));
                }
                _ => {}
            }
        }
        if let Some(v) = &self.next {
            v.handle(event, records, registration, query, timeouts, queue);
        }
    }
}
