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

impl TXTRecord{
    /// New TXT Record
    /// To properly create a new TXT Record struct provide String's in the format of `key=value`
    pub fn new(txt_record: Vec<String>) -> Result<Self, String>{

        for txt in &txt_record{
            if txt.split('=').count() != 2{
                return Err("Txt Record is not the incorrect Format. (key=value)".to_string())
            }
        }

        Ok(TXTRecord{txt_record})

    }
}

impl RData for TXTRecord {
    fn to_bytes(&self) -> Vec<u8> {
        //Prepend each string byte array with a byte indicating the length
        let mut result = vec![];
        for txt in &self.txt_record{
            let l = txt.len() as u8;
            result.push(l);
            result.extend(txt.as_bytes());
        }
        result
    }
}
