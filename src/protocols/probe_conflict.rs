/// Probe Tiebreak
///
/// Resolve conflict in case of probe response by others
///
/// This step is only available if MdnsResolver state is `State::Probing`
///
/// [RFC6762 Section 8.2 - Simultaneous Probe Tiebreak](https://www.rfc-editor.org/rfc/rfc6762#section-8.2)
/// - Compare two services by lexicographic greatness
/// - If greater, return `Ok(())`, else return `Error::ServiceOccupied`
/// - If we return an error, Resolver should wait 1s and restart probing
pub async fn probe_tiebreak() -> (){
    //TODO

  
}