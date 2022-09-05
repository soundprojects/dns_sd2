use std::{
    io::{self},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4},
    ops::BitAnd,
};

use bitvec::prelude::*;
use bytes::Bytes;
use futures::SinkExt;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use tokio::net::UdpSocket;
use tokio_util::{codec::BytesCodec, udp::UdpFramed};

use crate::{message::MdnsMessage, MdnsError, IP_ANY};

/// When there might be multiple responders on the system,
/// the port for UDP messages might be occupied without the REUSE_ADDR set
/// This may prevents us from receiving unicast UDP Messages
///
///
/// ## RFC Reference
/// - [RFC6762 Section 15.1 - Receiving Unicast Responses](https://www.rfc-editor.org/rfc/rfc6762#section-15.1)
///
/// ## Protocol
/// - Attempt to bind a UDP Socket to port 5353 without setting REUSE_ADDR
/// - If this fails, this means another program is already using this port
/// - Return [`MdnsError::AddressAlreadyTaken`]
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

/// Determine whether a query host is reachable
///
/// Compares the host IP addresses with the available interface IP addresses
///
/// ## RFC Reference
/// -[RFC6762 Section 11 - Source Address Check](https://www.rfc-editor.org/rfc/rfc6762#section-11)
///
/// # Example
///
/// `fd48:a12f:7b0c:3da8:0000:0000:0000:0000` and `fd48:a12f:7b0c:3da8:0000:0000:0000:abcd`
///  
/// should be in the same network if the subnet is the default 64 bit prefix
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

// Lexicographic Comparison
//
// Compares two records for which is lexicographically 'later'
//
// [RFC6762 Section 8.2 - Simultaneous Probe Tiebreak](https://www.rfc-editor.org/rfc/rfc6762#section-8.2)
//
// TODO Clarify protocol procedures
// Impl Ord for Service{}

///Send an Mdns Message to the multicast group with the given Socket
pub async fn send_message(
    socket: &mut UdpFramed<BytesCodec>,
    message: &MdnsMessage,
) -> std::io::Result<()> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 251)), 5353);

    socket
        .send((Bytes::from(message.to_bytes()), addr))
        .await
        .expect("Should send message");

    Ok(())
}
