// Subscribe to changes and reconnect after a problem has occured

//Parse incoming messages and sort them by mdns type
//Trigger events

// TTL decreasing mechanism

//Handle questions and resolves

//Implement a timeout mechanism which waits for either a timeout or a response
//Use select! with a counter for the retries

//Use mspc channel to make the crate wait for sending goodbye packets on closure

//Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::{
    io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddrV4},
    ops::BitAnd,
    time::Duration,
};

use bitvec::{prelude::Msb0, view::BitViewSized};
use futures::{stream::FuturesUnordered, StreamExt};
use message::MdnsMessage;
use protocols::handler::{Event, Handler};

use record::ResourceRecord;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

use thiserror::Error;
use tokio::{
    net::UdpSocket,
    select,
    sync::mpsc::{unbounded_channel, UnboundedSender},
    time::interval,
};
use tokio_util::{codec::BytesCodec, udp::UdpFramed};

use crate::protocols::probe::ProbeHandler;

//MULTICAST Constants
const IP_ANY: [u8; 4] = [0, 0, 0, 0];

pub mod header;
pub mod message;
pub mod protocols;
pub mod question;
pub mod record;
pub mod records;

pub enum ServiceState {
    Prelude,
    Probing,
    Announcing,
    Registered,
    ShuttingDown,
}

impl Default for ServiceState {
    fn default() -> Self {
        ServiceState::Probing
    }
}

#[derive(Debug, Error)]
pub enum MdnsError {
    #[error("Address is already taken")]
    AddressAlreadyTaken {
        #[from]
        source: io::Error,
    },
}

///PRELUDE FUNCTIONS
//

/// Check Unique Responder
///
/// When there might be multiple responders on the system,
/// the port for UDP messages might be occupied without the REUSE_ADDR set
/// This prevents us from receiving UDP Messages
///
/// This step is only available if MdnsResolver state is `State::Prelude`
///
/// [RFC6762 Section 15.1 - Receiving Unicast Responses](https://www.rfc-editor.org/rfc/rfc6762#section-15.1)
/// - Attempt to bind a UDP Socket to port 5353 without setting REUSE_ADDR
/// - If this fails, this means another program is already using this port
/// - Return Error:PortNotAvailable
///
/// # Example
///
/// In this example we consider our responder to be unique.
/// However, there might already be a MDNS Resolver running on the OS.
/// This will mean that `check_unique_responder()` will return `Err("Address is already taken")`
///
/// ```rust,no_run
/// use dns_sd2::check_unique_responder;
///
/// #[tokio::main]
/// async fn main(){
///
/// assert!(check_unique_responder().await.is_ok());
///
/// }
/// ```
///
pub async fn check_unique_responder() -> Result<(), MdnsError> {
    debug!("Checking for Unique Responder");

    //Create a udp ip4 socket
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    //Do not allow this port to be reused by other sockets to test if socket is already bound to
    socket.set_reuse_port(false)?;

    //Create IPV4 any adress
    let address = SocketAddrV4::new(IP_ANY.into(), 5353);

    //Bind to wildcard 0.0.0.0
    socket.bind(&SockAddr::from(address))?;

    debug!("Responder is unique!");

    Ok(())
}

/// UTILITY FUNCTIONS
//

/// Create Multicast Socket
///
/// Creates a Udp Ipv4 Multicast socket and binds it to the wildcard 0.0.0.0 address
pub fn create_socket() -> io::Result<UdpSocket> {
    //Create a udp ip4 socket
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    //Allow this port to be reused by other sockets
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;
    socket.set_nonblocking(true)?;

    //Create IPV4 any adress
    let address = SocketAddrV4::new(IP_ANY.into(), 5353);

    debug!("Created Address");

    //Bind to wildcard 0.0.0.0
    socket.bind(&SockAddr::from(address))?;

    debug!("Bound Socket");

    //Join multicast group
    socket.join_multicast_v4(&Ipv4Addr::new(224, 0, 0, 51), address.ip())?;

    info!("Joined Multicast");

    //Convert to std::net udp socket
    let udp_std_socket: std::net::UdpSocket = socket.into();

    //Convert to tokio udp socket
    let udp_socket = UdpSocket::from_std(udp_std_socket)?;

    info!(
        "Created a UDP Socket at {}, {}",
        address.ip().to_string(),
        address.port().to_string()
    );

    return Ok(udp_socket);
}

/// Is Reachable Ipv4
///
/// Determine whether a query host is reachable
///
/// Compares the host IP addresses with the available interface IP addresses
///
/// [RFC6762 Section 11 - Source Address Check](https://www.rfc-editor.org/rfc/rfc6762#section-11)
///
/// - Get the octets from the host and source Ip Address e.g. `[192,168,1,1]`
/// - Get the octets from the Subnet e.g. `[255,255,255,0]`
/// - Perform a bitwise AND operator (0 + 0 = 0, 0 + 1 = 0, 1 + 1 = 1) on both network addresses
/// - Host and source network should be equal in order to be reachable
///
/// # Example
///
/// `192.168.1.1` and `192.168.1.30` should be in the same network if the subnet is `255.255.255.0`
///  
/// ```rust
///
/// use std::net::Ipv4Addr;
///
/// use dns_sd2::is_reachable_ipv4;
///
/// assert!(is_reachable_ipv4(&Ipv4Addr::new(192,168,1,1), &Ipv4Addr::new(255,255,255,0), &Ipv4Addr::new(192,168,1,30)));
///
/// assert!(!is_reachable_ipv4(&Ipv4Addr::new(192,168,1,1), &Ipv4Addr::new(255,255,255,0), &Ipv4Addr::new(192,168,2,30)));

/// ```
pub fn is_reachable_ipv4(host_ip: &Ipv4Addr, host_subnet: &Ipv4Addr, source_ip: &Ipv4Addr) -> bool {
    let host_network = host_ip
        .octets()
        .into_bitarray::<Msb0>()
        .bitand(host_subnet.octets().into_bitarray());

    let source_network = source_ip
        .octets()
        .into_bitarray::<Msb0>()
        .bitand(host_subnet.octets().into_bitarray());

    host_network == source_network
}

/// Is Reachable Ipv6
///
/// Determine whether a query host is reachable
///
/// Compares the host IP addresses with the available interface IP addresses
///
/// [RFC6762 Section 11 - Source Address Check](https://www.rfc-editor.org/rfc/rfc6762#section-11)
///
/// # Example
///
/// `fd48:a12f:7b0c:3da8:0000:0000:0000:0000` and `fd48:a12f:7b0c:3da8:0000:0000:0000:abcd` should be in the same network if the subnet is the default 64 bit prefix
///
/// subnet = `ffff:ffff:ffff:ffff:0000:0000:0000`
///  
/// ```rust
///
/// use std::net::Ipv6Addr;
///
/// use dns_sd2::is_reachable_ipv6;
///
/// assert!(is_reachable_ipv6(&Ipv6Addr::new(0xfd48,0xa12f,0x7b0c,0x3da8,0,0,0,0), &Ipv6Addr::new(0xffff,0xffff,0xffff,0xffff,0,0,0,0), &Ipv6Addr::new(0xfd48,0xa12f,0x7b0c,0x3da8,0,0,0,0xabcd)));
///
/// assert!(!is_reachable_ipv6(&Ipv6Addr::new(0xfd48,0xa12f,0x7b0c,0x3da8,0,0,0,0), &Ipv6Addr::new(0xffff,0xffff,0xffff,0xffff,0,0,0,0), &Ipv6Addr::new(0xfd48,0xa12f,0x7b0c,0x3da9,0,0,0,0xabcd)));
/// ```
pub fn is_reachable_ipv6(host_ip: &Ipv6Addr, host_subnet: &Ipv6Addr, source_ip: &Ipv6Addr) -> bool {
    let host_network = host_ip
        .octets()
        .into_bitarray::<Msb0>()
        .bitand(host_subnet.octets().into_bitarray());

    let source_network = source_ip
        .octets()
        .into_bitarray::<Msb0>()
        .bitand(host_subnet.octets().into_bitarray());

    host_network == source_network
}

/// Lexicographic Comparison
///
/// Compares two records for which is lexicographically 'later'
///
/// [RFC6762 Section 8.2 - Simultaneous Probe Tiebreak](https://www.rfc-editor.org/rfc/rfc6762#section-8.2)
///
/// TODO Clarify protocol procedures
// Impl Ord for Service{}

#[derive(Default)]
pub struct Service {
    name: String,
    txt_records: Vec<String>,
    timeout: u64,
    state: ServiceState,
}

#[derive(Default)]
pub struct Query {
    name: String,
    timeout: u64,
}

#[derive(Default)]
pub struct DnsSd2 {
    records: Vec<ResourceRecord>,
    registrations: Vec<Service>,
    queries: Vec<Query>,
    tx: Option<UnboundedSender<Event>>,
}

impl<'a> DnsSd2 {
    pub fn handle<T: Handler<'a>>(&mut self, h: &T, event: &Event, timeouts: &mut Vec<u64>) {
        h.handle(
            event,
            &mut self.records,
            &mut self.registrations,
            &mut self.queries,
            timeouts,
        )
    }

    pub fn register(&mut self, name: String, txt_records: Vec<String>) {
        debug!("register");
        self.tx
            .as_ref()
            .unwrap()
            .send(Event::Register(name, txt_records))
            .expect("Failed to send with Tx");
    }
    /// Init
    ///
    /// Called by Client after creating a Dns_Sd2 Struct
    ///
    /// This starts the main event loop for the library and builds the chain of responsibility
    ///
    /// A select! loop picks between a 1s Interval Stream, a dynamic interval stream set by the chain and the UdpFramed Stream
    ///
    ///
    /// Creates a UDP IP4 Socket and binds to the 'any' 0.0.0.0 interface
    ///
    /// Allows the port to be reused
    ///
    /// Connect to Multicast group
    ///
    /// [DNS Specification](https://www.rfc-editor.org/rfc/rfc6762#section-8.1)
    pub async fn init(&mut self) -> io::Result<()> {
        pretty_env_logger::init_timed();

        info!("Initializing Event Loop");

        //Channel
        let (tx, mut rx) = unbounded_channel();

        self.tx = Some(tx);

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
                c = rx.recv() => {
                    debug!("{:?}", c);
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

async fn sleep_for(duration: u64) -> u64 {
    tokio::time::sleep(Duration::from_millis(duration)).await;
    duration
}
