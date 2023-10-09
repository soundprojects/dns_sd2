use packed_struct::prelude::*;

use crate::record::RData;

/// A Resource Record
///
///
///
///[1035 Section 3.4.1 - A RDATA format](https://www.rfc-editor.org/rfc/rfc1035#section-3.4.1)
#[derive(PackedStruct, Default, Clone, Debug)]
#[packed_struct(endian = "msb", bit_numbering = "msb0")]
pub struct ARecord {
    //IP    Ipv4 Address
    //      Hosts that have multiple internet addresses have multiple A records
    #[packed_field(bits = "0..=31")]
    pub ip: [u8; 4],
}

impl RData for ARecord {
    fn to_bytes(&self) -> Vec<u8> {
        self.pack().expect("Failed to pack A record").into()
    }
}
