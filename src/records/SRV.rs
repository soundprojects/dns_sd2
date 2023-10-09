use crate::{name::Name, record::RData};
/// SRV Resource Record
///
///
///
///[A DNS RR for specifying the location of services (DNS SRV)](https://www.rfc-editor.org/rfc/rfc2782)
#[derive(Clone, Debug)]
pub struct SRVRecord {
    //Priority Priority by which queriers contact host names within this service.
    //         Queriers MUST start with lowest priority. If equal, querier looks at Weight.
    //         Is used for indicating which server is preferred if there are multiple servers providing the same service
    pub priority: u16,
    //Weight   Weight is a second selection mechanism by which Queriers can determine which host to contact first
    //         This mechanism is out of scope for this crate
    pub weight: u16,
    //Port     Port on which the service handles traffic
    pub port: u16,
    //Target   The domain name of the target host. There MUST be one or more address records for this name and this name
    //         cannot be an alias. For example 'MyMac.local'
    pub target: Name,
}

impl RData for SRVRecord {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        //PRIORITY
        bytes.extend(self.priority.to_be_bytes());

        //WEIGHT
        bytes.extend(self.weight.to_be_bytes());

        //PORT
        bytes.extend(self.port.to_be_bytes());

        //TARGET
        bytes.extend(self.target.to_bytes());

        bytes
    }
}
