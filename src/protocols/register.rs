use crate::{message::MdnsMessage, record::ResourceRecord, service::ServiceState, Query, Service};

use super::handler::{Event, Handler};

/// Register MDNS Service
///
/// First step in MDNS announcement protocol
///
///
/// [RFC6762 Section 8.1 - Probing](https://www.rfc-editor.org/rfc/rfc6762#section-8.1)
/// - Wait for a 0-250ms time period to prevent simultaneous querying by devices on startup
/// - Query the service
/// - Wait for 250ms or get a response -> Return Conflict Error
/// - Query again
/// - Wait for 250ms or get a response -> Return Conflict Error
/// - Return Ok -> Service has not been registrered
///
/// This handler sets the Registration allowing the Probing handler to start the announcement process
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
        queue: &mut Vec<MdnsMessage>,
    ) {
        match event {
            Event::Register(host, service, protocol, port, txt_records) => {
                debug!(
                    "Added new Registration {}.{}.{}.local on port {} with txt_records {:?}",
                    host, service, protocol, port, txt_records
                );

                *registration = Some(Service {
                    host: host.to_string(),
                    service: service.to_string(),
                    protocol: protocol.to_string(),
                    port: port.to_owned(),
                    txt_records: txt_records.to_vec(),
                    ..Default::default()
                });
            }
            _ => {}
        }
        if let Some(v) = &self.next {
            v.handle(event, records, registration, query, timeouts, queue);
        }
    }
}

#[test]
fn test_registration_handler() {
    //Mock Registration
    let host: String = "TestMachine".into();
    let service: String = "_test".into();
    let protocol: String = "_tcp".into();
    let port = 53000;
    let txt_records: Vec<String> = vec![];

    let event = Event::Register(
        host.clone(),
        service.clone(),
        protocol.clone(),
        port.clone(),
        txt_records.clone(),
    );

    let handler = RegisterHandler::default();

    let mut registration = None;

    //Pass Registration into Handler
    handler.handle(
        &event,
        &mut vec![],
        &mut registration,
        &mut None,
        &mut vec![],
        &mut vec![],
    );

    assert!(registration.is_some());

    let result = registration.unwrap();
    assert_eq!(result.host, host);
    assert_eq!(result.service, service);
    assert_eq!(result.protocol, protocol);
    assert_eq!(result.txt_records, txt_records);
    assert_eq!(result.state, ServiceState::Prelude);
}
