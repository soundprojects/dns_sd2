// TODO UDP SOCKET STRUCTURE
// Create a UDP Socket for every available interface on some IP
// Join multicast group
// Subscribe to changes and reconnect after a problem has occured

//Parse incoming messages and sort them by mdns type
//Trigger events

//Send types

// TODO DNS Struct
// Message type
// Header
// TTL
// RRData

// TTL decreasing mechanism

//Handle questions and resolves
//Announcement and probing

//Implement a timeout mechanism which waits for either a timeout or a response
//Use select! with a counter for the retries

//Use mspc channel to make the crate wait for sending goodbye packets on closure

//Logging
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4},
};

use rand::Rng;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};

use tokio::net::UdpSocket;

//MULTICAST Constants
const IP_ANY: [u8; 4] = [0, 0, 0, 0];

/// UTILITY FUNCTIONS
//

/// Compress Name
///
/// Message compression for optimizing MDNS Records
///
/// [RFC6762 Section 18.14 - Name Compression](https://www.rfc-editor.org/rfc/rfc6762#section-18.14)
///
/// [RFC1035 Section 4.1.4 - Message Compression](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.4)
///
/// TODO Clarify protocol procedures
pub fn compress_name() -> String {
    todo!();
}

/// Decompress Name
///
/// Message decompression for optimizing MDNS Records
///
/// [RFC6762 Section 18.14 - Name Compression](https://www.rfc-editor.org/rfc/rfc6762#section-18.14)
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
// Impl Custom ordering here for Service

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

    //Spawn task for listening to messages

    //Send first probing

    debug!("Ready to probe");

    Ok(())
}
