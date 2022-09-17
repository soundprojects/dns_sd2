use packed_struct::PackedStruct;

use crate::{
    name::Name,
    question::{QClass, QType},
    records::{a::ARecord, aaaa::AAAARecord, ptr::PTRRecord, srv::SRVRecord},
};
use std::fmt::Debug;

/// A Record describing a certain [`QClass`] and [`QType`]
///
///## RFC Reference
/// - [RFC1035 Section 4.1.3 - Resource record format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.3)
///
/// ## Bit Layout
/// ```no_run
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
/// ```
#[derive(Debug)]
pub struct ResourceRecord {
    /// NAME     
    ///
    /// a domain name to which this record pertains
    ///
    /// is of variable length, padding is not applied
    /// maximum length is 255 octets
    /// ## RFC Specification
    /// [RFC1035 Section 4.1.2 - Question section format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.2)
    pub name: Name,
    /// TYPE      
    ///
    /// two octets containing one of the RR type codes.
    /// This field specifies the meaning of the data in the RDATA field
    pub record_type: QType,
    /// CLASS     
    ///
    /// two octets which specify the class of the data in the
    /// RDATA field. The first bit indicates whether to flush Cache for this record
    pub record_class: QClass,
    /// CACHE FLUSH
    ///
    /// Whether the cache flush bit (Top bit of QClass) is set and the receiving host
    /// should flush the caches for this record after 1 second
    pub cache_flush: bool,
    /// TTL    
    ///    
    /// a 32 bit unsigned integer that specifies the time
    /// interval (in seconds) that the resource record may be
    /// cached before it should be discarded.  Zero values are
    /// interpreted to mean that the RR can only be used for the
    /// transaction in progress, and should not be cached.
    pub ttl: u32,
    /// RDLENGTH
    ///
    /// an unsigned 16 bit integer that specifies the length in
    //  octets of the RDATA field
    pub rdlength: u16,
    /// RDATA     
    ///
    /// a variable length string of octets that describes the
    /// resource.  
    /// The format of this information varies
    /// according to the TYPE and CLASS of the resource record
    ///
    /// Implementation is done through the RData trait allowing methods for packing to a byte array
    /// See structs in the ./records folder
    pub rdata: Option<Box<dyn RData + Send>>,
}

impl ResourceRecord {
    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        //If there is no RDATA set return Error
        if let Some(rdata) = &self.rdata {
            let mut bytes = vec![];

            //NAME
            bytes.extend(self.name.to_bytes());

            //TYPE
            bytes.extend((self.record_type as u16).to_be_bytes());

            //CLASS
            let mut class_bytes = (self.record_class as u16).to_be_bytes();

            //If Caches need to be flushed, set first bit of Class to 1
            if self.cache_flush {
                class_bytes[0] |= 0b1000_0000;
            }
            bytes.extend(class_bytes);

            //TTL
            bytes.extend(self.ttl.to_be_bytes());

            //Retrieve the RData as bytes
            let rdata_bytes = rdata.to_bytes();
            let rdata_length = rdata_bytes.len() as u16;

            //RDLENGTH
            bytes.extend(rdata_length.to_be_bytes());

            //RDATA
            bytes.extend(rdata_bytes);

            Ok(bytes)
        } else {
            return Err("No RDATA set for this record".to_string());
        }
    }

    /// Create a 'A' type Resource Record
    pub fn create_a_record(name: Name, ip: [u8; 4]) -> Self {
        let rdata = ARecord { ip };

        let rdata_packed = rdata.pack().expect("Packing A record failed");

        ResourceRecord {
            name,
            record_type: QType::A,
            record_class: QClass::In,
            cache_flush: false,
            ttl: 60,
            rdlength: rdata_packed
                .len()
                .try_into()
                .expect("Could not cast usize to u16"),
            rdata: Some(Box::new(rdata)),
        }
    }

    /// Create a 'AAAA' type Resource Record
    pub fn create_aaaa_record(name: Name, ip: [u16; 4]) -> Self {
        let rdata = AAAARecord { ip };

        let rdata_packed = rdata.pack().expect("Packing AAAA record failed");

        ResourceRecord {
            name,
            record_type: QType::Aaaa,
            record_class: QClass::In,
            cache_flush: false,
            ttl: 120,
            rdlength: rdata_packed
                .len()
                .try_into()
                .expect("Could not cast usize to u16"),
            rdata: Some(Box::new(rdata)),
        }
    }

    /// Create a 'PTR' type Resource Record
    pub fn create_ptr_record(host: String, service: String, protocol: String) -> Self {
        let rdata = PTRRecord {
            name: Name::new(host.clone() + "." + &service + "." + &protocol + ".local")
                .expect("Should be valid"),
        };

        let rdata_packed = rdata.to_bytes();

        ResourceRecord {
            name: Name::new(service + "." + &protocol + ".local").expect("Should be valid"),
            record_type: QType::Ptr,
            record_class: QClass::In,
            cache_flush: false,
            ttl: 60,
            rdlength: rdata_packed
                .len()
                .try_into()
                .expect("Could not cast usize to u16"),
            rdata: Some(Box::new(rdata)),
        }
    }

    /// Create a 'SRV' type Resource Record
    pub fn create_srv_record(service: String, port: u16, target: String) -> Self {
        let rdata = SRVRecord {
            priority: 0,
            port,
            weight: 0,
            target: Name::new(target).expect("Should be valid"),
        };

        let rdata_packed = rdata.to_bytes();
        ResourceRecord {
            name: Name::new(service).expect("Should be valid"),
            record_type: QType::Srv,
            record_class: QClass::In,
            cache_flush: false,
            ttl: 60,
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
/// Allows for packing byte arrays from Resource Record Data
pub trait RData {
    fn to_bytes(&self) -> Vec<u8>;
}

///TODO TEST THIS
impl Debug for dyn RData + Send {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RData : {{{:?}}}", self)
    }
}
