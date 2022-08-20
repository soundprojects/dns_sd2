use bitvec::prelude::*;

/// MDNS Header Format
///
///
///
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
///[1035 Section 4.1.1 - Header Format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
#[derive(Default, Clone, Debug, Copy)]
pub struct Header {
    //ID        A 16 bit identifier assigned by the program that generates any kind of query
    //          This identifier is copied to the corresponding reply and can be used by the requester
    //          to match up replies to outstanding queries
    pub id: u16,
    //QR        A one bit field that specifies whether this message is a query (false) or a response (true)
    pub qr: bool,
    //OPCODE    A four bit field that specifies kind of query in this message. This value is set by the originator of a query
    //          and is copied into the response. The values are:
    //          0       a standard query (QUERY)
    //          1       an inverse query (IQUERY)
    //          2       a server status request (STATUS)
    //          3-15    for future use
    pub opcode: BitArray,
    //AA        Authoritative Answer - this bit is valid in responses,
    //          and specifies that the responding name server is an authority for the domain name in question
    //          Note that the contents of the answer section may have multiple owner names because of aliases
    //          The AA bit corresponds to the name which matches the query name, or the first owner name in the answer section
    pub aa: bool,
    //TC        TrunCation - Specifies that this message was truncated due to length greater than that permitted
    //          on the transmission channel
    pub tc: bool,
    //RD        Recursion Desired - This bit may be set in a query and is copied into the response. If RD is set,
    //          it directs the name server to pursue the query recursively. Recursive query support is optional.
    pub rd: bool,
    //RA        Recursion Available - this is set or cleared in a response and denotes whether
    //          recursive query support is available in the name server}
    pub ra: bool,
    //Z         Reserved for future use. Must be zero in all queries and responses
    pub z: BitArray,
    //RCODE     Response code - this 4 but field is set as part of responses. The values have the following interpretation:
    //          0   No error condition
    //          1   Format error - The name server was unable to interpret the query.
    //          2   Server failure - The name server was unable to process this query due to a problem with the name server.
    //          3   Name error - Meaningful only for responses from an authoritative name server. This code signifies that
    //                           domain name referenced in the query does not exist
    //          4   Not Implemented - The name server does not support this kind of query
    //          5   Refused - The name server refuses to perform the specified operation for policy reasons.
    //                        For example, a name server mmay not wish to provide the information to the particular requester,
    //                        or a name server may not wish to performm a particular operation (e.g. zone transfer) for particular data
    //          6-15 Reserved for future use
    pub rcode: BitArray,
    //QDCOUNT   an unsigned 16 bit integer specifying the number of entries in the question section
    pub qdcount: u16,
    //ANCOUNT   an unsigned 16 bit integer specifying the nummber of entries in the answer section
    pub ancount: u16,
    //NSCOUNT   an unsigned 16 bit integer specifying the number of name server resource records in the authority records section
    pub nscount: u16,
    //ARCOUNT   an unsigned 16 bit integer specifying the number of resource records in the additional records section.
}
