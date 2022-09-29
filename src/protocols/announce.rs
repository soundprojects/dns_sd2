use crate::{
    message::MdnsMessage, record::ResourceRecord, service::ServiceState, MdnsError, Query, Service,
};

use super::handler::{Event, Handler};

/// Announce MDNS Service
///
/// Second step in MDNS announcement protocol
///
/// ## RFC Reference
/// - [RFC6762 Section 8.3 - Announcing](https://www.rfc-editor.org/rfc/rfc6762#section-8.3)
///
/// ## Protocol
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
        registration: &mut Option<&mut Service>,
        query: &mut Option<Query>,
        timeouts: &mut Vec<(ServiceState, u64)>,
        queue: &mut Vec<MdnsMessage>,
    ) -> Result<(), MdnsError> {
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
                    queue.push(MdnsMessage::announce(r));
                    debug!("First Announcement Sent");
                    r.state = ServiceState::WaitForSecondAnnouncement;
                    timeouts.push((r.state, 1000));
                }
                ServiceState::SecondAnnouncement => {
                    queue.push(MdnsMessage::announce(r));
                    debug!("Second Announcement Sent, REGISTERED");
                    r.state = ServiceState::Registered;
                }
                _ => {}
            }
        }
        if let Some(v) = &self.next {
            v.handle(event, records, registration, query, timeouts, queue)?;
        }
        Ok(())
    }
}

#[test]
fn test_announce_handler() {
    //Mock Service
    //Result if Registration Handler worked properly
    let mut service = Service {
        host: "TestMachine".into(),
        service: "_test".into(),
        protocol: "_tcp".into(),
        port: 53000,
        txt_records: vec![],
        state: ServiceState::FirstAnnouncement,
    };

    let handler = AnnouncementHandler::default();

    //Pass into Handler
    //Step 1: Send Announcement, Should add first timeout with interval 1000 ms
    let mut timeouts = vec![];
    let mut queue = vec![];
    handler
        .handle(
            &Event::Ttl(),
            &mut vec![],
            &mut Some(&mut service),
            &mut None,
            &mut timeouts,
            &mut queue,
        )
        .unwrap();

    assert_eq!(timeouts.len(), 1);
    assert_eq!(timeouts[0].1, 1000);
    assert_eq!(timeouts[0].0, ServiceState::WaitForSecondAnnouncement);
    assert_eq!(queue.len(), 1);

    timeouts.clear();
}
