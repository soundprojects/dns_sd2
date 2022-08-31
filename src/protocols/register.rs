use crate::{record::ResourceRecord, service::ServiceState, Query, Service};

use super::handler::{Event, Handler};

/// Probe MDNS Service
///
/// First step in MDNS announcement protocol
///
/// This step is only available if MdnsResolver state is `State::Probing`
///
/// [RFC6762 Section 8.1 - Probing](https://www.rfc-editor.org/rfc/rfc6762#section-8.1)
/// - Wait for a 0-250ms time period to prevent simultaneous querying by devices on startup
/// - Query the service
/// - Wait for 250ms or get a response -> Return Conflict Error
/// - Query again
/// - Wait for 250ms or get a response -> Return Conflict Error
/// - Return Ok -> Service has not been registrered
///
#[derive(Default, Copy, Clone)]
pub struct RegisterHandler<'a> {
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> Handler<'a> for RegisterHandler<'a> {
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
    ) {
        match event {
            Event::Register(n, t) => {
                debug!("Added new Registration {} with txt_records {:?}", n, t);

                *registration = Some(Service {
                    name: n.to_string(),
                    txt_records: t.to_vec(),
                    ..Default::default()
                });
            }
            _ => {}
        }
        if let Some(v) = &self.next {
            v.handle(event, records, registration, query, timeouts);
        }
    }
}
