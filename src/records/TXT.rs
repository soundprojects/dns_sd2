use crate::record::RData;
/// TXT Resource Record
///
///
///
///[1035 Section 3.3.14 - TXT Record format](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.14)
#[derive(Default, Clone, Debug)]
pub struct TXTRecord {
    //TXT-RECORD    One or more <character-string>s
    //              Holds data in the form of `key=value`
    pub txt_record: Vec<String>,
}

impl RData for TXTRecord {
    fn to_bytes(&self) -> Vec<u8> {
        self.txt_record
    }

    fn parse(&self) -> Option<Box<dyn RData + Send>> {
        None
    }
}
