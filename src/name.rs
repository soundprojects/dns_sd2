/// Name is a wrapper to provide
/// methods to properly support division of name into labels
/// which are properly serialized with prepending lengths and
/// terminating zero octet
#[derive(Debug, Clone, Default)]
pub struct Name {
    /// String content
    content: String,
}

impl Name {
    pub fn new(name: String) -> Result<Name, String> {
        Ok(Name { content: name })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        let labels: Vec<_> = self.content.split('.').collect();

        //Names are made up of labels prepended with their lengths
        //Or with pointers (See Name Compression Handler)
        //Name is terminated by a zero length Octet
        //[RFC1035 Section 4.1.2 - Question section format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.2)
        for label in labels {
            bytes.push(label.len() as u8);
            bytes.extend(label.as_bytes());
        }

        //Name must end with a zero Octet
        bytes.push(0);

        bytes
    }
}
