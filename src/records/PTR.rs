use crate::{name::Name, record::RData};

/// PTR Resource Record
///
///
///
///[1035 Section 3.3.12 - PTR Record format](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.12)
#[derive(Default, Clone, Debug)]
pub struct PTRRecord {
    //Name     A <domain-name> which points to some location in the domain name space
    //         Requires no additional record processing
    pub name: Name,
}

impl RData for PTRRecord {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        //Name
        bytes.extend(self.name.to_bytes());

        bytes
    }
}
