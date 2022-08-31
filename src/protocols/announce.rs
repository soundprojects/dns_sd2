use crate::{record::ResourceRecord, service::ServiceState, Query, Service};

use super::handler::{Event, Handler};

/// Announce MDNS Service
///
/// Second step in MDNS announcement protocol
///
/// This step is only available if MdnsResolver state is `State::Announcing`
///
/// [RFC6762 Section 8.3 - Announcing](https://www.rfc-editor.org/rfc/rfc6762#section-8.3)
/// - Send unsollicited response with all answers, both shared and unique
/// - For the unique records, set cache flush bit to '1'
/// - Wait 1s
/// - Send unsollicited response again
#[derive(Default, Copy, Clone)]
pub struct AnnouncementHandler<'a> {
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> Handler<'a> for AnnouncementHandler<'a> {
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
        if let Some(r) = registration {
            //TIMEOUTS
            match event {
                Event::TimeElapsed((s, _t)) => {
                    //States must match with registered timeouts
                    if *s == r.state {
                        match s {
                            ServiceState::WaitForSecondAnnouncement => {
                                r.state = ServiceState::SecondAnnouncement
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            //STATE MANAGEMENT
            match r.state {
                ServiceState::FirstAnnouncement => {
                    //First Announcement Here
                    debug!("First Announcement Sent");
                    r.state = ServiceState::WaitForSecondAnnouncement;
                    timeouts.push((r.state, 1000));
                }
                ServiceState::SecondAnnouncement => {
                    //Send Second Announcement Here
                    debug!("Second Announcement Sent, REGISTERED");
                    r.state = ServiceState::Registered;
                }
                _ => {}
            }
        }
        if let Some(v) = &self.next {
            v.handle(event, records, registration, query, timeouts);
        }
    }
}
