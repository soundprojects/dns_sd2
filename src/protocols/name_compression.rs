/// Compress Name
///
/// Message compression for optimizing MDNS Records
///
/// This compression means that names which are repeated in records are replaced by a pointer
/// to the first place where this name appears. The pointer is an octet which has the first two bits set, followed
/// by the offset indicating the place where we can find the original name
///
/// Labels start with the first two bits set to zero
///
/// Compression is only applied to RR where the format is specified:
/// CNAME NS MX A AAAA PTR
///
/// Name compression SHOULD NOT be applied to SRV Records
///
/// [RFC6762 Section 18.14 - Name Compression](https://www.rfc-editor.org/rfc/rfc6762#section-18.14)
///
/// [RFC1035 Section 4.1.4 - Message Compression](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.4)
///
/// Split the domain into parts and calculate lengths per label
pub fn compress_name(_message: &BitArray) -> BitArray {
    todo!();
}

/// Decompress Name
///
/// Message decompression for optimizing MDNS Records
///
/// [RFC6762 Section 18.14 - Name Compression](https://www.rfc-editor.org/rfc/rfc6762#section-18.14)
///
/// [RFC1035 Section 4.1.4 - Message Compression](https://www.rfc-editor.org/rfc/rfc1035#section-4.1.4)
///
/// TODO Clarify protocol procedures
pub fn decompress_name() -> String {
    todo!();
}
