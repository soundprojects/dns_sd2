use packed_struct::prelude::*;

/// MDNS Header Format
///
///
/// Uses [`PackedStruct`] for packing with bit layout as shown below:
///
/// ## Bit Layout
/// ```no_run
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
///```
///## RFC Reference
///- [1035 Section 4.1.1 - Header Format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
#[derive(PackedStruct, Default, Clone, Debug)]
#[packed_struct(endian = "msb", bit_numbering = "msb0")]
pub struct Header {
    //ID        A 16 bit identifier assigned by the program that generates any kind of query
    //          This identifier is copied to the corresponding reply and can be used by the requester
    //          to match up replies to outstanding queries
    #[packed_field(bits = "0..=15")]
    pub id: u16,
    //QR        A one bit field that specifies whether this message is a query (false) or a response (true)
    #[packed_field(bits = "16")]
    pub qr: bool,
    //OPCODE    A four bit field that specifies kind of query in this message. This value is set by the originator of a query
    //          and is copied into the response. The values are:
    //          0       a standard query (QUERY)
    //          1       an inverse query (IQUERY)
    //          2       a server status request (STATUS)
    //          3-15    for future use
    #[packed_field(bits = "17..=20", ty = "enum")]
    pub opcode: OpCode,
    //AA        Authoritative Answer - this bit is valid in responses,
    //          and specifies that the responding name server is an authority for the domain name in question
    //          Note that the contents of the answer section may have multiple owner names because of aliases
    //          The AA bit corresponds to the name which matches the query name, or the first owner name in the answer section
    #[packed_field(bits = "21")]
    pub aa: bool,
    //TC        TrunCation - Specifies that this message was truncated due to length greater than that permitted
    //          on the transmission channel
    #[packed_field(bits = "22")]
    pub tc: bool,
    //RD        Recursion Desired - This bit may be set in a query and is copied into the response. If RD is set,
    //          it directs the name server to pursue the query recursively. Recursive query support is optional.
    #[packed_field(bits = "23")]
    pub rd: bool,
    //RA        Recursion Available - this is set or cleared in a response and denotes whether
    //          recursive query support is available in the name server}
    #[packed_field(bits = "24")]
    pub ra: bool,
    //Z         Reserved for future use. Must be zero in all queries and responses
    #[packed_field(bits = "25..=27")]
    pub z: Integer<u8, packed_bits::Bits<3>>,
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
    #[packed_field(bits = "28..=31", ty = "enum")]
    pub rcode: RCode,
    //QDCOUNT   an unsigned 16 bit integer specifying the number of entries in the question section
    #[packed_field(bits = "32..=47")]
    pub qdcount: u16,
    //ANCOUNT   an unsigned 16 bit integer specifying the nummber of entries in the answer section
    #[packed_field(bits = "48..=63")]
    pub ancount: u16,
    //NSCOUNT   an unsigned 16 bit integer specifying the number of name server resource records in the authority records section
    #[packed_field(bits = "64..=79")]
    pub nscount: u16,
    //ARCOUNT   an unsigned 16 bit integer specifying the number of resource records in the additional records section.
    #[packed_field(bits = "80..=95")]
    pub arcount: u16,
}

impl Header {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.pack().expect("Failed to pack Header").into()
    }
}

///A four bit field that specifies the kind of query in this message. This value is set by the originator of a query
/// and is copied into the response. The values are:
///         -  0       a standard query (QUERY)
///         - 1       an inverse query (IQUERY)
///         - 2       a server status request (STATUS)
///         - 3-15    for future use
///
///## RFC Reference
///- [1035 Section 4.1.1 - Header Format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum OpCode {
    StandardQuery = 0,
    InverseQuery = 1,
    ServerStatusRequest = 2,
}

///Default Implementation for OpCode
///
/// Default is a Standard Query (QUERY)
impl Default for OpCode {
    fn default() -> Self {
        OpCode::StandardQuery
    }
}

///This 4 bit field is set as part of responses. The values have the following interpretation:
///          - 0   No error condition
///          - 1   Format error - The name server was unable to interpret the query.
///          - 2   Server failure - The name server was unable to process this query due to a problem with the name server.
///          - 3   Name error - Meaningful only for responses from an authoritative name server. This code signifies that
///                           domain name referenced in the query does not exist
///          - 4   Not Implemented - The name server does not support this kind of query
///          - 5   Refused - The name server refuses to perform the specified operation for policy reasons.
///                        For example, a name server mmay not wish to provide the information to the particular requester,
///                        or a name server may not wish to performm a particular operation (e.g. zone transfer) for particular data
///          - 6-15 Reserved for future use
///
///## RFC Reference
///[1035 Section 4.1.1 - Header Format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1)
#[derive(PrimitiveEnum_u8, Clone, Copy, Debug, PartialEq)]
pub enum RCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

///Default Implementation for OpCode
///
/// Default is a Standard Query (QUERY)
impl Default for RCode {
    fn default() -> Self {
        RCode::NoError
    }
}

impl Header {
    /// New Header
    ///
    /// Returns a `Header` with default values of an empty query with id of 0
    /// zero record counts and all flags set to false
    pub fn new() -> Self {
        Header::default()
    }
}

#[test]
fn test_header() {
    let header = Header::new();

    //Test that the Header packs into a byte array correctly
    assert!(header.pack().is_ok());

    //Test that the unpacked Header is 12 bytes in length
    assert!(header.pack().unwrap().len() == 12);
}
