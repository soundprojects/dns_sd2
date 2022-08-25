/// Question section format
///
/// [RFC1035 Section 4.1.2 - Question section format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.2)
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                     QNAME                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     QTYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     QCLASS                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive(Clone, Debug)]
pub struct Question {
    //Name      a domain name represented as a sequence of labels, where
    //          each label consists of a length octet followed by that
    //          number of octets.  The domain name terminates with the
    //          zero length octet for the null label of the root.  Note
    //          that this field may be an odd number of octets; no
    //          padding is used.
    //
    //          Maximum size is 255 octets
    //
    //          [RFC1035 Section 2.3.4 - Question section format](https://www.rfc-editor.org/rfc/rfc1035#section-2.3.4)
    pub name: String,
    //Type     Defines what type of resource the question is asking for
    pub qtype: QType,
    //Class     Defines what network class the question is asking for
    pub qclass: QClass,
}

/// DNS QClass
///
/// QClass defines what network class the question is asking for
///
/// QClass are a superset of Class, so all Class are valid QClass
///
/// This field is used in Queries and Resource Records
///
/// [RFC1035 Section 3.2.5 - CLASS Values](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.5)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QClass {
    // 1 The Internet
    In = 1,
    // 2 CSNet class (Obsolete)
    Cs = 2,
    // 3 The Chaos class
    Ch = 3,
    // 4 Hesiod (Dyer 87)
    Hs = 4,
    //255 Any Class
    Any = 255,
}

/// DNS QType
///
/// QType defines what the question is asking for
///
/// QTypes are a superset of Types, so all Types are valid QTypes
///
/// This field is used in the Question section of the MDNS Message
///
/// [RFC1035 Section 3.2.2 - DNS Types](https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2)
/// [RFC1035 Section 4.1 - Format](https://www.rfc-editor.org/rfc/rfc1035#section-4.1)

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum QType {
    // 1 a host address (IPV4)
    A = 1,
    // 2 an authoritative name server
    Ns = 2,
    // 3 a mail destination (OBSOLETE - use MX)
    Md = 3,
    // 4 a mail forwarder (OBSOLETE - use MX)
    Mf = 4,
    // 5 the canonical name for an alias
    Cname = 5,
    // 6 marks the start of a zone of authority
    Soa = 6,
    // 7 a mailbox domain name (EXPERIMENTAL)
    Mb = 7,
    // 8 a mail group member (EXPERIMENTAL)
    Mg = 8,
    // 9 a mail rename domain name (EXPERIMENTAL)
    Mr = 9,
    // 10 a null RR (EXPERIMENTAL)
    Null = 10,
    // 11 a well known description
    Wks = 11,
    // 12 a domain name parser
    Ptr = 12,
    // 13 a host information
    Hinfo = 13,
    //14 mailbox or mail list information
    Minfo = 14,
    // 15 mail exchange
    Mx = 15,
    //16 text strings
    Txt = 16,
    // 28 a host address (IPV6)
    Aaaa = 28,
    // 33 a service record
    Srv = 33,
    // 252 A request for a transfer of an entire zone
    Axfr = 252,
    // * A request for all records
    Any = 255,
}
