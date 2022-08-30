//Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use async_stream::try_stream;
use futures::{stream::FuturesUnordered, Stream, StreamExt};
use message::MdnsMessage;
use protocols::handler::{Event, Handler};
use record::ResourceRecord;
use service::{Query, Service};
use std::{io, time::Duration};
use thiserror::Error;
use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    time::interval,
};
use tokio_util::{codec::BytesCodec, udp::UdpFramed};

use crate::{protocols::probe::ProbeHandler, utility::create_socket};

const IP_ANY: [u8; 4] = [0, 0, 0, 0];

pub mod header;
pub mod message;
pub mod protocols;
pub mod question;
pub mod record;
pub mod records;
pub mod service;
pub mod utility;

#[derive(Debug, Error)]
pub enum MdnsError {
    #[error("Address is already taken")]
    AddressAlreadyTaken {
        #[from]
        source: io::Error,
    },
}

/// DnsSd2
///
/// Main library struct
///
/// Records:        Contains a Vec of ResourceRecord's currently active on the network
/// Registrations:  May contain a registered Service
/// Query:          May contain an active search
/// Tx.Rx:          Channel for communicating (closing)
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

impl<'a> DnsSd2 {
    pub fn handle<T: Handler<'a>>(&mut self, h: &T, event: &Event, timeouts: &mut Vec<u64>) {
        h.handle(
            event,
            &mut self.records,
            &mut self.registration,
            &mut self.query,
            timeouts,
        )
    }

    /// Registers a Mdns Service
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
    ) -> impl Stream<Item = Result<Service, String>> + '_ {
        debug!(
            "Register Service {} with TXT Records {:?}",
            name, txt_records
        );

        self.tx
            .send(Event::Register(name, txt_records))
            .expect("Failed to send with Tx");

        self.init().await
    }

    ///Browse for a Mdns Service
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
    ) -> impl Stream<Item = Result<Service, String>> + '_ {
        debug!("Browse for Service {}", name);

        self.tx
            .send(Event::Browse(name))
            .expect("Failed to send with Tx");

        self.init().await
    }

    /// Called by `browse()` or `register()` to run main loop
    ///
    /// This starts the main event loop for the library and builds the chain of responsibility
    ///
    /// A select! loop picks between a 1s Interval Stream, a dynamic interval stream set by the chain and the UdpFramed Stream
    ///
    /// Returns a stream for the requested Enum
    pub async fn init(&mut self) -> impl Stream<Item = Result<Service, String>> + '_ {
        pretty_env_logger::init_timed();

        info!("Initializing Event Loop");

        try_stream! {
                //Socket
                let udp_socket = create_socket().expect("Failed to create socket");

                let mut frame = UdpFramed::new(udp_socket, BytesCodec::new());

                //Chain of responsibility
                let probe = ProbeHandler::default();

                //Collection of timer futures
                let mut dynamic_interval = FuturesUnordered::new();

                dynamic_interval.push(sleep_for(2000));

                //Normal 1s TTL Timer
                let mut interval = interval(Duration::from_secs(1));

                loop {
                    let result = select! {
                        _ = frame.next() => {
                            Event::Message(MdnsMessage::default())
                        }
                        c = self.rx.recv() => {
                            debug!("{:?}", c);
                            let s = Service::default();
                            yield s;
                            c.expect("Should contain an Event")
                        }
                        t = dynamic_interval.next(), if !dynamic_interval.is_empty() => {
                            debug!("Timed out for {:?} ms", t);
                            Event::TimeElapsed(t.unwrap_or_default())
                        }
                        _ = interval.tick() => {
                            debug!("Tick");
                            Event::TimeElapsed(1000)

                        }
                    };

                    let mut timeouts = vec![];

                    //Execute the chain
                    self.handle(&probe, &result, &mut timeouts);

                    //Add the resulting timeouts from the chain to our dynamic interval futures
                    for timeout in timeouts {
                        dynamic_interval.push(sleep_for(timeout));
                    }
                }
        }
    }
}

async fn sleep_for(duration: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(duration)).await;
    duration
}
