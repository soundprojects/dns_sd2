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
};

use bitvec::{
    prelude::{BitArray, Msb0},
    view::BitViewSized,
};
use message::MdnsMessage;
use rand::Rng;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

use thiserror::Error;
use tokio::net::UdpSocket;

//MULTICAST Constants
const IP_ANY: [u8; 4] = [0, 0, 0, 0];

pub mod header;
pub mod message;
pub mod question;
pub mod record;
pub mod records;
pub mod service;

pub enum ServiceState {
    Prelude,
    Probing,
    Announcing,
    Registered,
    ShuttingDown,
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
    socket.set_reuse_address(false)?;

    //Create IPV4 any adress
    let address = SocketAddrV4::new(IP_ANY.into(), 5353);

    //Bind to wildcard 0.0.0.0
    socket.bind(&SockAddr::from(address))?;

    debug!("Responder is unique!");

    Ok(())
}

/// Handle Query
///
/// When a query is received on the interface, it is handled through this function
///
/// - Determine if caches need to be flushed (with 1s timeout)
///
/// [RFC6762 Section 10.2 - Announcements to Flush Outdated Cache Entries](https://www.rfc-editor.org/rfc/rfc6762#section-10.2)
///
/// - Determine if this query is a query we are preparing ourselves
///
/// [RFC6762 Section 7.3 - Duplicate Question Supression](https://www.rfc-editor.org/rfc/rfc6762#section-7.3)
///
/// - Determine if there is passive failure (lack of response after this query where we would have expected it)

pub async fn handle_query(_query: &MdnsMessage) -> io::Result<()> {
    todo!();
}

/// Handle Response
///
/// When a response is received on the interface, it is handled through this function
///
/// - Determine if this message is truncated
/// - Defer response by 400-500 ms to allow for more known answers to be received
///
/// [RFC6762 Section 7.2 - Multicast Known Answer Supression](https://www.rfc-editor.org/rfc/rfc6762#section-7.2)
///
/// - Determine if TTL of known answers is less than half of the correct TTL -> do not include record
///
/// [RFC6762 Section 7.1 - Multicast Known Answer Supression](https://www.rfc-editor.org/rfc/rfc6762#section-7.2)
///
/// - Determine if caches need to be flushed (with 1s timeout)
///
/// [RFC6762 Section 10.2 - Announcements to Flush Outdated Cache Entries](https://www.rfc-editor.org/rfc/rfc6762#section-10.2)
///
/// - Determine if this is a goodbye packet (TTL of 0)
/// - Set TTL to 1 so service is removed after 1 second
///
/// [RFC6762 Section 10.1 - Goodbye Packets](https://www.rfc-editor.org/rfc/rfc6762#section-10.1)
///
/// - Determine if this is an update or a possible conflict
///
/// - Determine if this is a response we are preparing ourselves
///
/// [RFC6762 Section 7.4 - Duplicate Answer Supression](https://www.rfc-editor.org/rfc/rfc6762#section-7.4)

pub async fn handle_response(_response: &MdnsMessage) -> io::Result<()> {
    todo!();
}

/// UTILITY FUNCTIONS
//


/// Create Multicast Socket
/// 
/// Creates a Udp Ipv4 Multicast socket and binds it to the wildcard 0.0.0.0 address
pub fn create_socket() -> io::Result<UdpSocket>{
    //Create a udp ip4 socket
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    //Allow this port to be reused by other sockets
    socket.set_reuse_address(true)?;

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

/// Compress Name
///
/// Message compression for optimizing MDNS Records
///
/// This compression means that names which are repeated in records are replaced by a pointer
/// to the first place where this name appears. The pointer is an octet which has the first two bits set, followed
/// by the offset indicating the place where we can find the original name
///
/// Labels start with the first two bits set to zero
///
/// Compression is only applied to RR where the format is specified:
/// CNAME NS MX A AAAA PTR
///
/// Name compression SHOULD NOT be applied to SRV Records
///
/// [RFC6762 Section 18.14 - Name Compression](https://www.rfc-editor.org/rfc/rfc6762#section-18.14)
///
/// [RFC1035 Section 4.1.4 - Message Compression](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.4)
///
/// Split the domain into parts and calculate lengths per label
pub fn compress_name(_message: &BitArray) -> BitArray {
    todo!();
}

/// Decompress Name
///
/// Message decompression for optimizing MDNS Records
///
/// [RFC6762 Section 18.14 - Name Compression](https://www.rfc-editor.org/rfc/rfc6762#section-18.14)
///
/// [RFC1035 Section 4.1.4 - Message Compression](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.4)
///
/// TODO Clarify protocol procedures
pub fn decompress_name() -> String {
    todo!();
}

/// Lexicographic Comparison
///
/// Compares two records for which is lexicographically 'later'
///
/// [RFC6762 Section 8.2 - Simultaneous Probe Tiebreak](https://www.rfc-editor.org/rfc/rfc6762#section-8.2)
///
/// TODO Clarify protocol procedures
// Impl Ord for Service{}

/// PROBING AND ANNOUNCING FUNCTIONS
//

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
pub async fn probe() -> io::Result<()> {
    let random_delay = rand::thread_rng().gen_range(0..250);
    tokio::time::sleep(std::time::Duration::from_millis(random_delay)).await;

    //TODO Query the service

    //TODO Select statement with receiving and parsing / timer
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    //TODO Select statement with receiving and parsing / timer
    tokio::time::sleep(std::time::Duration::from_millis(250)).await;

    Ok(())
}

/// Probe Tiebreak
///
/// Resolve conflict in case of probe response by others
///
/// This step is only available if MdnsResolver state is `State::Probing`
///
/// [RFC6762 Section 8.2 - Simultaneous Probe Tiebreak](https://www.rfc-editor.org/rfc/rfc6762#section-8.2)
/// - Compare two services by lexicographic greatness
/// - If greater, return `Ok(())`, else return `Error::ServiceOccupied`
/// - If we return an error, Resolver should wait 1s and restart probing
pub async fn probe_tiebreak() -> io::Result<()> {
    //TODO

    Ok(())
}

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
pub async fn announce() -> io::Result<()> {
    let random_delay = rand::thread_rng().gen_range(0..250);
    tokio::time::sleep(std::time::Duration::from_millis(random_delay)).await;

    //TODO Send unsollicited response

    //TODO Select statement with receiving and parsing / timer
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

    //TODO Send unsollicited response

    Ok(())
}

///SHUTDOWN FUNCTIONS
//

/// Send Goodbye Packets
///
/// Last step in MDNS shutdown protocol
///
/// When a service is dropped, send a goodbye record so other hosts know this service is gone
///
/// This step is only available if MdnsResolver state is `State::ShuttingDown`
///
/// [RFC6762 Section 10.1 - Goodbye Packets](https://www.rfc-editor.org/rfc/rfc6762#section-10.1)
/// - Send unsollicited response with a TTL of 0
pub async fn goodbye() -> io::Result<()> {
    todo!();
}

/// Something
///
/// Creates a UDP IP4 Socket and binds to the 'any' 0.0.0.0 interface
///
/// Allows the port to be reused
///
/// Connect to Multicast group
///
/// [DNS Specification](https://www.rfc-editor.org/rfc/rfc6762#section-8.1)
pub async fn something() -> io::Result<()> {
    pretty_env_logger::init_timed();

    info!("Running something");

    let _udp_socket = create_socket();

    //Spawn task for listening to messages

    //Send first probing

    debug!("Ready to probe");

    Ok(())
}
