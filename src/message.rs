use crate::{
    enums::{Question, ResourceRecord},
    header::Header,
};

/// MDNS Message
///
/// Message struct for an MDNS Message
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
    //Additionsals  See Additionals.rs
    pub additionals: Vec<ResourceRecord>,
}
