use super::handler::{Event, Handler};
use crate::{
    message::MdnsMessage, record::ResourceRecord, service::ServiceState, MdnsError, Query, Service,
};
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
                    debug!(
                        "Adding Timeout for Probing {}.{}.{}.local",
                        r.host, r.service, r.protocol
                    );
                    r.state = ServiceState::WaitForFirstProbe;
                    timeouts.push((r.state, thread_rng().gen_range(0..250)));
                }
                ServiceState::FirstProbe => {
                    debug!(
                        "Sending Probe Query for {}.{}.{}.local",
                        r.host, r.service, r.protocol
                    );
                    queue.push(MdnsMessage::probe(&r));
                    r.state = ServiceState::WaitForSecondProbe;
                    timeouts.push((r.state, 250));
                }
                ServiceState::SecondProbe => {
                    debug!(
                        "Sending second Probe Query for {}.{}.{}.local",
                        r.host, r.service, r.protocol
                    );
                    queue.push(MdnsMessage::probe(&r));
                    r.state = ServiceState::WaitForAnnouncing;
                    timeouts.push((r.state, 250));
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
fn test_probe_handler() {
    //Mock Service
    //Result if Registration Handler worked properly
    let mut service = Service {
        host: "TestMachine".into(),
        service: "_test".into(),
        protocol: "_tcp".into(),
        port: 53000,
        txt_records: vec![],
        state: ServiceState::Prelude,
    };

    let handler = ProbeHandler::default();

    //Pass into Handler
    //Step 1: Should add first timeout with interval 0-250 ms
    let mut timeouts = vec![];

    handler
        .handle(
            &Event::Ttl(),
            &mut vec![],
            &mut Some(&mut service),
            &mut None,
            &mut timeouts,
            &mut vec![],
        )
        .unwrap();

    assert_eq!(timeouts.len(), 1);
    assert!(timeouts[0].1 > 0);
    assert!(timeouts[0].1 < 250);
    assert_eq!(timeouts[0].0, ServiceState::WaitForFirstProbe);

    timeouts.clear();

    //Step 2: First probe finished change state
    service.state = ServiceState::WaitForFirstProbe;
    handler
        .handle(
            &Event::TimeElapsed((ServiceState::WaitForFirstProbe, 250)),
            &mut vec![],
            &mut Some(&mut service),
            &mut None,
            &mut timeouts,
            &mut vec![],
        )
        .unwrap();

    assert_eq!(service.state, ServiceState::FirstProbe);
    timeouts.clear();

    //Step 3: Should add second timeout with interval 250 ms
    service.state = ServiceState::FirstProbe;

    handler
        .handle(
            &Event::Ttl(),
            &mut vec![],
            &mut Some(&mut service),
            &mut None,
            &mut timeouts,
            &mut vec![],
        )
        .unwrap();

    assert_eq!(timeouts.len(), 1);
    assert_eq!(timeouts[0].1, 250);
    assert_eq!(timeouts[0].0, ServiceState::WaitForSecondProbe);
}
