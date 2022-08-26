use packed_struct::prelude::*;

use crate::record::RData;

/// AAAA Resource Record
///
///
///
///[3596 Section 2.1 - AAAA Record Format](https://www.rfc-editor.org/rfc/rfc3596#section-2.1)
#[derive(PackedStruct, Default, Clone, Debug)]
#[packed_struct(endian = "msb", bit_numbering = "msb0")]
pub struct AAAARecord {
    //IP    Ipv4 Address
    //      Hosts that have multiple internet addresses have multiple A records
    #[packed_field(bits = "0..=63")]
    pub ip: [u16; 4],
}

impl RData for AAAARecord {
    fn to_bytes(&self) -> Vec<u8> {
        self.pack().expect("Failed to pack AAAA record").into()
    }

    fn parse(&self) -> Option<Box<dyn RData + Send>> {
        None
    }
}
