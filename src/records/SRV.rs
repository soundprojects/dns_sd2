use crate::question::QClass;

use crate::record::RData;
/// SRV Resource Record
///
///
///
///[A DNS RR for specifying the location of services (DNS SRV)](https://www.rfc-editor.org/rfc/rfc2782)
#[derive(Clone, Debug)]
pub struct SRVRecord {
    //Service  A symbolic name for the desired service
    //         Is preprended with a '_' to prevent conflicts with naturally occuring labels
    pub service: String,
    //Proto    Protocol label, preprended with a '_'. Most commonly '_tcp' or '_udp'.
    pub proto: String,
    //Name     Domain the Resoruce Record refers to. Common example is '.local'
    pub name: String,
    //TTL      Standard Time to Live field. For SRV record the default is 120 Seconds.
    pub ttl: u32,
    //Classs   Standard QClass field
    pub class: QClass,
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
    //         cannot be an alias
}

impl RData for SRVRecord {
    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn parse(&self) -> Option<Box<dyn RData + Send>> {
        None
    }
}
