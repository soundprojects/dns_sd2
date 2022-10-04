use crate::{
    message::MdnsMessage, record::ResourceRecord, service::ServiceState, MdnsError, Query, Service,
};

use super::handler::{Event, Handler};

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
/// - Decrease TTL for each record by 1s
/// - Verify if TTL cache rules are met
/// - Notify if new query is necessary
#[derive(Default, Copy, Clone)]
pub struct UpdateTTLHandler<'a> {
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> Handler<'a> for UpdateTTLHandler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
    fn handle(
        &self,
        event: &Event,
        records: &mut Vec<ResourceRecord>,
        registration: &mut Option<&mut Service>,
        query: &mut Option<Query>,
        timeouts: &mut Vec<(ServiceState, u64)>,
        queue: &mut Vec<MdnsMessage>,
    ) -> Result<(), MdnsError> {
        match event {
            Event::Ttl {} => {
                records.iter_mut().for_each(|rec| {
                    if rec.ttl > 0 {
                        rec.ttl -= 1;
                    }

                    //TODO Add query signal here if rules are met
                });
            }
            _ => {}
        }
        if let Some(v) = &self.next {
            v.handle(event, records, registration, query, timeouts, queue)?;
        }

        Ok(())
    }
}
