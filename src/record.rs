use packed_struct::PackedStruct;

use crate::{
    question::{QClass, QType},
    records::{a::ARecord, aaaa::AAAARecord, ptr::PTRRecord, srv::SRVRecord},
};
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

impl ResourceRecord {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    pub fn create_a_record(name: String, ip: [u8; 4]) -> Self {
        let rdata = ARecord { ip };

        let rdata_packed = rdata.pack().expect("Packing A record failed");

        ResourceRecord {
            name,
            record_type: QType::A,
            record_class: QClass::In,
            ttl: 120,
            rdlength: rdata_packed
                .len()
                .try_into()
                .expect("Could not cast usize to u16"),
            rdata: Some(Box::new(rdata)),
        }
    }

    pub fn create_aaaa_record(name: String, ip: [u16; 4]) -> Self {
        let rdata = AAAARecord { ip };

        let rdata_packed = rdata.pack().expect("Packing AAAA record failed");

        ResourceRecord {
            name,
            record_type: QType::Aaaa,
            record_class: QClass::In,
            ttl: 120,
            rdlength: rdata_packed
                .len()
                .try_into()
                .expect("Could not cast usize to u16"),
            rdata: Some(Box::new(rdata)),
        }
    }

    pub fn create_ptr_record(name: String) -> Self {
        let rdata = PTRRecord { name: name.clone() };

        let rdata_packed = rdata.to_bytes();

        ResourceRecord {
            name,
            record_type: QType::Ptr,
            record_class: QClass::In,
            ttl: 120,
            rdlength: rdata_packed
                .len()
                .try_into()
                .expect("Could not cast usize to u16"),
            rdata: Some(Box::new(rdata)),
        }
    }

    pub fn create_srv_record(
        service: String,
        protocol: String,
        ttl: u32,
        port: u16,
        domain: String,
    ) -> Self {
        let rdata = SRVRecord {
            service,
            proto: protocol,
            priority: 0,
            ttl,
            class: QClass::In,
            port: 0,
            weight: 0,
            name: domain,
        };

        let rdata_packed = rdata.to_bytes();

        ResourceRecord {
            name: service,
            record_type: QType::Ptr,
            record_class: QClass::In,
            ttl: 120,
            rdlength: rdata_packed
                .len()
                .try_into()
                .expect("Could not cast usize to u16"),
            rdata: Some(Box::new(rdata)),
        }
    }
}

/// RData Trait
///
/// Trait describing functions for the RData field of a Resource Record
/// Allows for packing and unpacking byte arrays in and from Resource Records
pub trait RData {
    fn to_bytes(&self) -> Vec<u8>;

    fn parse(&self) -> Option<Box<dyn RData + Send>>;
}

///TODO TEST THIS
impl Debug for dyn RData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RData : {{{:?}}}", self)
    }
}
