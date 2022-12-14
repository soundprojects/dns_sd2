//Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use async_stream::try_stream;
use futures::{executor::block_on, stream::FuturesUnordered, Stream, StreamExt};
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
    },
    utility::{create_socket, send_message},
};

const IP_ANY: [u8; 4] = [0, 0, 0, 0];

pub mod header;
pub mod message;
pub mod name;
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
    #[error("Service name is already taken")]
    NameAlreadyTaken {},
    #[error("Service was removed")]
    ServiceRemoved {},
    #[error("Closing")]
    Closing {},
    #[error("Invalid Mdns Message")]
    InvalidMessage {},
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
    pub tx: UnboundedSender<Event>,
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
    /// Drop DnsSd2
    ///
    /// When dropped or when receiving [`Event::Closing{}`]
    /// Sends out Goodbye Packets if client initiated with [`DnsSd2::register()`]
    /// To properly unregister a [`Service`] on the network
    fn drop(&mut self) {
        debug!("Dropping DnsSd2");
        let handler = GoodbyeHandler::default();
        //Socket
        let udp_socket = create_socket().expect("Failed to create socket");

        let mut frame = UdpFramed::new(udp_socket, BytesCodec::new());

        let mut queue = vec![];

        if self
            .handle(&handler, &Event::Closing(), &mut vec![], &mut queue)
            .is_ok()
        {
            //Note: We block here because Drop must be synchronous
            for message in queue {
                block_on(send_message(&mut frame, &message)).expect("Failed to send goodbye");
            }
        }
    }
}

impl<'a> DnsSd2 {
    /// Runs Chain of Responsibility for this client
    ///
    /// This function is called in the [`DnsSd2::init()`] loop
    /// Each part of the chain handles a different part of the MDNS Protocol
    ///
    /// Should return `Ok(())` or it propogates an [`MdnsError`]
    /// Mutates records, registration, query and timeouts depending on Handler input
    pub fn handle<T: Handler<'a>>(
        &mut self,
        h: &T,
        event: &Event,
        timeouts: &mut Vec<(ServiceState, u64)>,
        queue: &mut Vec<MdnsMessage>,
    ) -> Result<(), MdnsError> {
        let mut registration = None;
        if self.registration.is_some() {
            registration = self.registration.as_mut();
        }
        h.handle(
            event,
            &mut self.records,
            &mut registration,
            &mut self.query,
            timeouts,
            queue,
        )?;
        Ok(())
    }

    /// Registers an Mdns [`Service`]
    ///
    /// ## Example
    ///
    /// ```rust, ignore
    /// use dns_sd2::Dns_Sd2;
    ///
    /// let stream = client.register("_myservice._udp.local".into(), vec![]).await;
    ///
    /// //This is necessary to iterate the Stream
    /// pin_mut!(stream);
    ///
    /// while let Some(Ok(s)) = stream.next().await {
    ///     debug!("Registered a service {:?}", s);
    /// }
    /// ```
    pub async fn register(
        &mut self,
        host: String,
        service: String,
        protocol: String,
        port: u16,
        txt_records: Vec<String>,
    ) -> impl Stream<Item = Result<Service, MdnsError>> + '_ {
        debug!(
            "Register Service {}.{}.{}.local with port {} with TXT Records {:?}",
            host, service, protocol, port, txt_records
        );

        self.tx
            .send(Event::Register(host, service, protocol, port, txt_records))
            .expect("Failed to send with Tx");

        self.init().await
    }

    /// Browse for an Mdns [`Service`]
    ///
    /// ## Example
    ///
    /// ```rust, ignore
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
                let mut probe_handler = ProbeHandler::default();
                let mut announcement_handler = AnnouncementHandler::default();
                let goodbye_handler = GoodbyeHandler::default();

                //Set Chain Order from back to front
                announcement_handler.set_next(&goodbye_handler);
                probe_handler.set_next(&announcement_handler);


                //Collection of timer futures
                let mut timeouts = FuturesUnordered::new();
                //Normal 1s TTL Timer
                let mut interval = interval(Duration::from_secs(1));

                loop {
                    let result = select! {
                        //Received a message on the Socket
                        _ = frame.next() => {
                            Event::Message(MdnsMessage::default())
                        }
                        //Received a Command from the client
                        c = self.rx.recv() => {
                            c.expect("Should contain a Command")
                        }
                        //Close signal handler
                        _close = tokio::signal::ctrl_c() => {
                            debug!("Ctrl C! Closing");
                            Event::Closing()
                        }
                        //A dynamic timeout has finished
                        t = timeouts.next(), if !timeouts.is_empty() => {
                            debug!("Timed out for {:?} ms", t);
                            Event::TimeElapsed(t.unwrap_or_default())
                        }
                        //TTL 1s timer has ticked
                        _ = interval.tick() => {
                            Event::Ttl()
                        }
                    };

                    //Check for specific command or signals
                    match &result{
                        Event::Register(host, service, protocol, port, txt_records) => {
                            self.registration = Some(Service{host: host.into(), service: service.into(), protocol: protocol.into(), port: *port, txt_records: txt_records.to_vec(), state: ServiceState::Prelude})
                        }
                        Event::Closing{} => {return}
                        _ => {}
                    }

                    //Fill a Vec with new timeouts and a Vec with a queue of messages we will send with the socket
                    let mut new_timeouts = vec![];
                    let mut queue = vec![];


                    //Execute the chain
                    self.handle(&probe_handler, &result, &mut new_timeouts, &mut queue)?;

                    let s = Service::default();
                    yield s;

                    //Add the resulting timeouts from the chain to our dynamic interval futures
                    for (s, t) in new_timeouts {
                        timeouts.push(sleep_for(s,t));
                    }

                    //Send the messages in the queue with our socket
                    for message in queue{
                        send_message(&mut frame, &message).await.expect("Should send Message");
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
