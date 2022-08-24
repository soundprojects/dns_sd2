use crate::{
    enums::Question,
    header::Header,
    record::ResourceRecord
};

/// MDNS Message
///
/// Message struct for an MDNS Message
/// 
/// UDP Messages may not exceed 512 octets
/// If the message is larger, the message needs to be split with the truncated flag set for all but the last message
///
/// See linked files for information about the content
///
/// [1035 Section 4.1 - Format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1)
pub struct MdnsMessage {
    //Header        See Header.rs
    pub header: Header,
    //Questions     See Question.rs
    pub questions: Vec<Question>,
    //Answers       See Answers.rs
    pub answers: Vec<ResourceRecord>,
    //Authorities   See Authorities.rs
    pub authorities: Vec<ResourceRecord>,
    //Additionals       Additional records which the responder might consider useful in addition to its answers
    //                  For example, the responder might send its known A and AAAA records when answering to a SRV Question
    //                  To prevent unnecessary latency and extra querying for the querier
    pub additionals: Vec<ResourceRecord>,
}
