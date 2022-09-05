use super::handler::{Event, Handler};
use crate::{message::MdnsMessage, record::ResourceRecord, service::ServiceState, Query, Service};
use rand::{thread_rng, Rng};

/// Probe MDNS Service
///
/// First step in MDNS announcement protocol
///
/// This step is only available if MdnsResolver state is `State::Probing`
///
///
/// ## RFC Reference
/// - [RFC6762 Section 8.1 - Probing](https://www.rfc-editor.org/rfc/rfc6762#section-8.1)
///
/// ## Protocol
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
            //TIMEOUTS
            match event {
                Event::TimeElapsed((s, _t)) => {
                    //States must match with registered timeouts
                    if *s == r.state {
                        match s {
                            ServiceState::WaitForFirstProbe => r.state = ServiceState::FirstProbe,
                            ServiceState::WaitForSecondProbe => r.state = ServiceState::SecondProbe,
                            ServiceState::WaitForAnnouncing => {
                                r.state = ServiceState::FirstAnnouncement
                            }

                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            //STATE MANAGEMENT
            match r.state {
                ServiceState::Prelude => {
                    debug!("Adding Timeout for Probing {}", r.name);
                    r.state = ServiceState::WaitForFirstProbe;
                    timeouts.push((r.state, thread_rng().gen_range(0..250)));
                }
                ServiceState::FirstProbe => {
                    debug!("Sending Probe Query for {}", r.name);
                    queue.push(MdnsMessage::question(&r.name));
                    r.state = ServiceState::WaitForSecondProbe;
                    timeouts.push((r.state, 250));
                }
                ServiceState::SecondProbe => {
                    debug!("Sending Second Probe Query for {}", r.name);
                    queue.push(MdnsMessage::question(&r.name));
                    r.state = ServiceState::WaitForAnnouncing;
                    timeouts.push((r.state, 250));
                }
                _ => {}
            }
        }

        if let Some(v) = &self.next {
            v.handle(event, records, registration, query, timeouts, queue);
        }
    }
}
