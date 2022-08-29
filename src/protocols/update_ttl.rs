    /// Update TTL
    ///
    /// Update TTL Values for the given records
    ///
    ///
    /// [RFC6762 Section 10 - Resource Record TTL Values and Cache Coherency](https://www.rfc-editor.org/rfc/rfc6762#section-10)
    ///
    /// Most DNS TTL are set to a 75 minute default
    /// Other responses where the host name is equal to the record name (A, AAAA, SRV) are set to 120 seconds
    /// When the TTL default is down by 80%, a new query is necessary and 85, 90 and 95%.
    /// There should be a 2% offset of the TTL query to prevent simultaneous queries by multiple systems
    ///
    /// Only records that are of an active interest to a local client are in need of this cache maintenance
    /// [RFC6762 Section 5.2 - Continuous Multicast DNS Querying](https://www.rfc-editor.org/rfc/rfc6762#section-5.2)
    ///
    /// - Decrease TTL for each record by 1
    /// - Verify if TTL cache rules are met
    /// - Notify if new query is necessary
    pub fn update_ttl(&mut self) -> () {
        todo!();
    }