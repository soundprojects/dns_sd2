//Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use async_stream::try_stream;
use futures::{stream::FuturesUnordered, Stream, StreamExt};
use message::MdnsMessage;
use protocols::handler::{Event, Handler};
use record::ResourceRecord;
use service::{Query, Service, ServiceState};
use std::{io, time::Duration};
use thiserror::Error;
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::interval,
};
use tokio_util::{codec::BytesCodec, udp::UdpFramed};

use crate::{
    protocols::{
        announce::AnnouncementHandler, goodbye_packet::GoodbyeHandler, probe::ProbeHandler,
        register::RegisterHandler,
    },
    utility::create_socket,
};

const IP_ANY: [u8; 4] = [0, 0, 0, 0];

pub mod header;
pub mod message;
pub mod protocols;
pub mod question;
pub mod record;
pub mod records;
pub mod service;
pub mod utility;

///Mdns Error Types
#[derive(Debug, Error)]
pub enum MdnsError {
    #[error("Address is already taken")]
    AddressAlreadyTaken {
        #[from]
        source: io::Error,
    },
    #[error("Closing")]
    Closing {},
}

/// Construct DnsSd2 to allow for searching and registering services
///
/// ## Arguments
///
/// Attribute | Explanation
/// :--|:--
/// Records | Contains a Vec of [`ResourceRecord`] currently active on the network
/// Registrations | May contain a registered [`Service`]
/// Query | May contain an active search
/// Tx.Rx | Channel for communicating (closing)
///
/// ## Example
///
/// ```no_run
/// use dns_sd2::DnsSd2;
///
/// let client = DnsSd2::default();
///
/// ```
pub struct DnsSd2 {
    records: Vec<ResourceRecord>,
    registration: Option<Service>,
    query: Option<Query>,
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,
}

impl Default for DnsSd2 {
    fn default() -> Self {
        let (tx, rx) = unbounded_channel();

        Self {
            records: Default::default(),
            registration: Default::default(),
            query: Default::default(),
            tx,
            rx,
        }
    }
}

impl Drop for DnsSd2 {
    fn drop(&mut self) {
        debug!("Dropping DnsSd2");
        let handler = GoodbyeHandler::default();
        self.handle(&handler, &Event::Closing(), &mut vec![]);
    }
}

impl<'a> DnsSd2 {
    pub fn handle<T: Handler<'a>>(
        &mut self,
        h: &T,
        event: &Event,
        timeouts: &mut Vec<(ServiceState, u64)>,
    ) {
        h.handle(
            event,
            &mut self.records,
            &mut self.registration,
            &mut self.query,
            timeouts,
        )
    }

    /// Registers an Mdns [`Service`]
    ///
    /// ## Example
    ///
    /// ```rust, no_run
    /// use dns_sd2::Dns_Sd2;
    ///
    /// let stream = client.register("_myservice._udp.local".into(), vec![]).await;
    ///
    /// //This is necessary to iterate the Stream
    /// pin_mut!(stream);
    ///
    /// while let Some(Ok(s)) = stream.next().await {
    ///     debug!("Found a service {:?}", s);
    /// }
    /// ```
    pub async fn register(
        &mut self,
        name: String,
        txt_records: Vec<String>,
    ) -> impl Stream<Item = Result<Service, MdnsError>> + '_ {
        debug!(
            "Register Service {} with TXT Records {:?}",
            name, txt_records
        );

        self.tx
            .send(Event::Register(name, txt_records))
            .expect("Failed to send with Tx");

        self.init().await
    }

    /// Browse for an Mdns [`Service`]
    ///
    /// ## Example
    ///
    /// ```rust, no_run
    /// use dns_sd2::Dns_Sd2;
    ///
    /// let stream = client.browse("_services._udp.local".into()).await;
    ///
    /// //This is necessary to iterate the Stream
    /// pin_mut!(stream);
    ///
    /// while let Some(Ok(s)) = stream.next().await {
    ///     debug!("Found a service {:?}", s);
    /// }
    /// ```
    pub async fn browse(
        &mut self,
        name: String,
    ) -> impl Stream<Item = Result<Service, MdnsError>> + '_ {
        debug!("Browse for Service {}", name);

        self.tx
            .send(Event::Browse(name))
            .expect("Failed to send with Tx");

        self.init().await
    }

    /// Called by [`browse()`] or [`register()`] to run main loop
    ///
    /// This starts the main event loop for the library and builds the chain of responsibility
    ///
    /// A select! loop picks between a 1s Interval Stream, a dynamic interval stream set by the chain and the UdpFramed Stream
    ///
    /// Returns a stream for registration or search
    pub async fn init(&mut self) -> impl Stream<Item = Result<Service, MdnsError>> + '_ {
        info!("Initializing Event Loop");

        try_stream! {
                //Socket
                let udp_socket = create_socket().expect("Failed to create socket");

                let mut frame = UdpFramed::new(udp_socket, BytesCodec::new());

                //Chain of responsibility
                let mut register_handler = RegisterHandler::default();
                let mut probe_handler = ProbeHandler::default();
                let mut announcement_handler = AnnouncementHandler::default();
                let goodbye_handler = GoodbyeHandler::default();

                //Set Chain Order from back to front
                announcement_handler.set_next(&goodbye_handler);
                probe_handler.set_next(&announcement_handler);
                register_handler.set_next(&probe_handler);

                //Collection of timer futures
                let mut timeouts = FuturesUnordered::new();
                //Normal 1s TTL Timer
                let mut interval = interval(Duration::from_secs(1));

                loop {
                    let result = select! {
                        _ = frame.next() => {
                            Event::Message(MdnsMessage::default())
                        }
                        c = self.rx.recv() => {
                            let s = Service::default();
                            debug!("M");
                            yield s;
                            c.expect("Should contain an Event")
                        }
                        t = timeouts.next(), if !timeouts.is_empty() => {
                            debug!("Timed out for {:?} ms", t);
                            Event::TimeElapsed(t.unwrap_or_default())
                        }
                        _ = interval.tick() => {
                            Event::Ttl()

                        }
                    };

                    let mut new_timeouts = vec![];

                    //Execute the chain
                    self.handle(&register_handler, &result, &mut new_timeouts);

                    //Add the resulting timeouts from the chain to our dynamic interval futures
                    for (s, t) in new_timeouts {
                        timeouts.push(sleep_for(s,t));
                    }
                }
        }
    }
}

/// Sleep for a certain duration
///
/// Pass along the [`ServiceState`] for identification of finished timeouts in the  [`Handler`] chain
async fn sleep_for(state: ServiceState, duration: u64) -> (ServiceState, u64) {
    tokio::time::sleep(Duration::from_millis(duration)).await;
    (state, duration)
}
