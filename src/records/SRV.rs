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
    //         Is preprended with a '_' to prevent conflicts with naturally occuring labels For example '_airplay'
    pub service: String,
    //Proto    Protocol label, preprended with a '_'. For example '_udp' or '_tcp'
    pub proto: String,
    //Name     Domain the Resource Record refers to. Common example is '.local'
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
    //         cannot be an alias. For example 'MyMac.local'
    pub target: String,
}

impl RData for SRVRecord {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // //SERVICE
        // bytes.push(self.service.len() as u8);
        // bytes.extend(self.service.as_bytes());

        // //PROTO
        // bytes.push(self.proto.len() as u8);
        // bytes.extend(self.proto.as_bytes());

        // //NAME
        // bytes.push(self.name.len() as u8);
        // bytes.extend(self.name.as_bytes());

        // //TTL
        // bytes.extend(self.ttl.to_be_bytes());

        // //CLASS
        // bytes.extend((self.class as u16).to_be_bytes());

        //PRIORITY
        bytes.extend(self.priority.to_be_bytes());

        //WEIGHT
        bytes.extend(self.weight.to_be_bytes());

        //PORT
        bytes.extend(self.port.to_be_bytes());

        //TARGET
        bytes.push(self.target.len() as u8);
        bytes.extend(self.target.as_bytes());

        bytes
    }
}
