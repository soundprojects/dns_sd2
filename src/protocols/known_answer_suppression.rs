// /// Handle Query
// ///
// /// When a query is received on the interface, it is handled through this function
// ///
// /// - Determine if caches need to be flushed (with 1s timeout)
// ///
// /// [RFC6762 Section 10.2 - Announcements to Flush Outdated Cache Entries](https://www.rfc-editor.org/rfc/rfc6762#section-10.2)
// ///
// /// - Determine if this query is a query we are preparing ourselves
// ///
// /// [RFC6762 Section 7.3 - Duplicate Question Supression](https://www.rfc-editor.org/rfc/rfc6762#section-7.3)
// ///
// /// - Determine if there is passive failure (lack of response after this query where we would have expected it)

// pub async fn handle_query(_query: &MdnsMessage) -> io::Result<()> {
//     todo!();
// }

// /// Handle Response
// ///
// /// When a response is received on the interface, it is handled through this function
// ///
// /// - Determine if this message is truncated
// /// - Defer response by 400-500 ms to allow for more known answers to be received
// ///
// /// [RFC6762 Section 7.2 - Multicast Known Answer Supression](https://www.rfc-editor.org/rfc/rfc6762#section-7.2)
// ///
// /// - Determine if TTL of known answers is less than half of the correct TTL -> do not include record
// ///
// /// [RFC6762 Section 7.1 - Multicast Known Answer Supression](https://www.rfc-editor.org/rfc/rfc6762#section-7.2)
// ///
// /// - Determine if caches need to be flushed (with 1s timeout)
// ///
// /// [RFC6762 Section 10.2 - Announcements to Flush Outdated Cache Entries](https://www.rfc-editor.org/rfc/rfc6762#section-10.2)
// ///
// /// - Determine if this is a goodbye packet (TTL of 0)
// /// - Set TTL to 1 so service is removed after 1 second
// ///
// /// [RFC6762 Section 10.1 - Goodbye Packets](https://www.rfc-editor.org/rfc/rfc6762#section-10.1)
// ///
// /// - Determine if this is an update or a possible conflict
// ///
// /// - Determine if this is a response we are preparing ourselves
// ///
// /// [RFC6762 Section 7.4 - Duplicate Answer Supression](https://www.rfc-editor.org/rfc/rfc6762#section-7.4)

// pub async fn handle_response(_response: &MdnsMessage) -> io::Result<()> {
//     todo!();
// }
