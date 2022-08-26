/// PTR Resource Record
///
///
///
///[1035 Section 3.3.12 - PTR Record format](https://www.rfc-editor.org/rfc/rfc1035#section-3.3.12)
#[derive(Default, Clone, Debug)]
pub struct PTRRecord {
    //Name     A <domain-name> which points to some location in the domain name space
    //         Requires no additional record processing
    pub name: String,
}
