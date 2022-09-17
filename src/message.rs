use crate::{
    header::Header,
    name::Name,
    question::{QClass, QType, Question},
    record::ResourceRecord,
    service::Service,
};

/// Message struct for an MDNS Message
///
/// UDP Messages may not exceed 512 octets
/// If the message is larger, the message needs to be split with
/// the truncated flag set for all but the last message
///
/// See linked files for information about the content
///
///## RFC Reference
/// [1035 Section 4.1 - Format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1)
///
/// ## Example
///
/// ```
/// use dns_sd2::message::MdnsMessage;
///
/// let message = MdnsMessage::default();
/// ```
#[derive(Default, Debug)]
pub struct MdnsMessage {
    ///Header        See Header.rs
    pub header: Header,
    ///Questions     See Question.rs
    pub questions: Vec<Question>,
    ///Answers          
    /// Answer records which provide answers to the query questions as a response OR
    /// are delivered along a query as 'known answers'
    /// Known answers are used to prevent unnecessary responses by the responder,
    /// If the TTL of the responder is at least half the TTL of the known querier answer, the responder SHOULD NOT send this answer
    /// Queriers should not include known answers where the TTL is less than half of the original TTL
    /// ## RFC Specification
    /// [6762 Section 7.1 - Known-Answer Suppression](https://www.rfc-editor.org/rfc/rfc1035#section-7.1)
    pub answers: Vec<ResourceRecord>,
    /// Authorities
    /// For these records, the responder is the authority of this data and the 'creator' of this data
    /// Only responders which are the authority should send these records instead of from their cache
    /// The Authorities section is used when Probing to indicate it wishes to use these records when probing is succesfull
    ///
    /// Note: This section should not be confused with the AA (Authoritive Answer) bit in the Header section
    /// This bit can only be se set for responses while the authorities section is filled when querying and responding during probing
    /// ## RFC Specification
    /// [6762 Section 8.2 - Simultaneous Probe Tiebreaking](https://www.rfc-editor.org/rfc/rfc1035#section-8.2)
    ///
    pub authorities: Vec<ResourceRecord>,
    /// Additionals       
    /// Additional records which the responder might consider useful in addition to its answers
    /// For example, the responder might send its known A and AAAA records when answering to a SRV Question
    /// To prevent unnecessary latency and extra querying for the querier
    pub additionals: Vec<ResourceRecord>,
}

impl MdnsMessage {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        //HEADER
        bytes.extend(self.header.to_bytes());

        //QUESTIONS
        for question in &self.questions {
            bytes.extend(question.to_bytes());
        }

        //ANSWERS
        for answer in &self.answers {
            if let Ok(record) = answer.to_bytes() {
                bytes.extend(record)
            }
        }

        //AUTHORITIES
        for authority in &self.authorities {
            if let Ok(record) = authority.to_bytes() {
                bytes.extend(record)
            }
        }

        //ADDITONALS
        for additional in &self.additionals {
            if let Ok(record) = additional.to_bytes() {
                bytes.extend(record)
            }
        }

        bytes
    }

    /// Create a MdnsMessage for probing
    ///
    /// returns a Query type message requesting ANY class and type
    /// for the given service name
    ///
    /// Set as unicast (QU) so responding hosts can directly defend records
    /// The answer section SHOULD be empty  
    ///
    /// The authorities section SHOULD be filled with ALL the proposed records  
    /// These records are used in case of Probe Tiebreaking
    pub fn probe(service: &Service) -> MdnsMessage {
        let mut message = MdnsMessage::default();
        message.questions.push(Question {
            name: Name::new(service.host.clone() + ".local").expect("Should be valid"),
            qtype: QType::Any,
            qclass: QClass::Any,
            unicast_question: true,
        });
        message.header.qdcount = 1;

        let srv = ResourceRecord::create_srv_record(
            service.host.clone() + "." + &service.service + "." + &service.protocol + ".local",
            service.port,
            service.host.clone() + ".local",
        );

        let a = ResourceRecord::create_a_record(
            Name::new(service.host.clone() + ".local").expect("Should be valid"),
            [192, 168, 1, 123],
        );

        message.authorities.push(srv);

        message.authorities.push(a);

        message.header.nscount = 2;

        message
    }

    pub fn announce(service: &Service) -> MdnsMessage {
        let mut message = MdnsMessage::default();

        message.header.qr = true;
        message.header.aa = true;

        let ptr = ResourceRecord::create_ptr_record(
            service.host.clone(),
            service.service.clone(),
            service.protocol.clone(),
        );

        let mut srv = ResourceRecord::create_srv_record(
            service.host.clone() + "." + &service.service + "." + &service.protocol + ".local",
            service.port,
            service.host.clone() + ".local",
        );

        srv.cache_flush = true;

        let mut a = ResourceRecord::create_a_record(
            Name::new(service.host.clone() + ".local").expect("Should be valid"),
            [192, 168, 178, 19],
        );

        a.cache_flush = true;

        message.answers.push(ptr);

        message.answers.push(srv);

        message.answers.push(a);

        message.header.ancount = 3;

        message
    }

    pub fn goodbye(service: &Service) -> MdnsMessage {
        let mut message = MdnsMessage::default();

        message.header.qr = true;
        message.header.aa = true;

        let mut ptr = ResourceRecord::create_ptr_record(
            service.host.clone(),
            service.service.clone(),
            service.protocol.clone(),
        );

        ptr.ttl = 0;
        let mut srv = ResourceRecord::create_srv_record(
            service.host.clone() + "." + &service.service + "." + &service.protocol + ".local",
            service.port,
            service.host.clone() + ".local",
        );

        srv.ttl = 0;

        let mut a = ResourceRecord::create_a_record(
            Name::new(service.host.clone() + ".local").expect("Should be valid"),
            [192, 168, 178, 19],
        );

        a.ttl = 0;

        message.answers.push(ptr);

        message.answers.push(srv);

        message.answers.push(a);

        message.header.ancount = 3;

        message
    }
}
