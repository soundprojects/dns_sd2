use crate::enums::{QClass, QType};
use std::fmt::Debug;

/// [RFC1035 Section 4.1.3 - Resource record format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.3)
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                                               /
/// /                      NAME                     /
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     CLASS                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TTL                      |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                   RDLENGTH                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
/// /                     RDATA                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive(Debug)]
pub struct ResourceRecord {
    //NAME      a domain name to which this record pertains
    //
    //          is of variable length, padding is not applied
    //          maximum length is 255 octets
    //
    // [RFC1035 Section 4.1.2 - Question section format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.2)
    pub name: String,
    //TYPE      two octets containing one of the RR type codes.
    //          This field specifies the meaning of the data in the RDATA field
    pub record_type: QType,
    //CLASS     two octets which specify the class of the data in the
    //          RDATA field
    pub record_class: QClass,
    //TTL       a 32 bit unsigned integer that specifies the time
    //          interval (in seconds) that the resource record may be
    //          cached before it should be discarded.  Zero values are
    //          interpreted to mean that the RR can only be used for the
    //          transaction in progress, and should not be cached.
    pub ttl: u32,
    //RDLENGTH  an unsigned 16 bit integer that specifies the length in
    //          octets of the RDATA field
    pub rdlength: u16,
    //RDATA     a variable length string of octets that describes the
    //          resource.  The format of this information varies
    //          according to the TYPE and CLASS of the resource record
    //
    //          Implementation is done through the RData trait allowing methods for packing to a byte array
    //          See structs in the ./records folder
    pub rdata: Option<Box<dyn RData>>,
}

/// RData Trait
///
/// Trait describing functions for the RData field of a Resource Record
/// Allows for packing and unpacking byte arrays in and from Resource Records
pub trait RData {
    fn pack(&self) {}

    fn unpack(&self) {}
}

///TODO TEST THIS
impl Debug for dyn RData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RData : {{{:?}}}", self)
    }
}
