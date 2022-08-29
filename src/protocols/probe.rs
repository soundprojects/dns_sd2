use crate::{record::ResourceRecord, Service, Query};

use super::handler::{Handler, Event};


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
pub struct ProbeHandler<'a> {
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> Handler<'a> for ProbeHandler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
    fn handle(&self, event: &Event, records: &mut Vec<ResourceRecord>, registrations: &mut Vec<Service>, searches: &mut Vec<Query>, timeouts: &mut Vec<u64>) {
        
        match event{
            Event::TimeElapsed(a) => if *a == 1000 as u64{timeouts.push(100);},
            _ => {}
        }
        if let Some(v) = &self.next {
            v.handle(event, records, registrations, searches, timeouts);
        }
    }
}
