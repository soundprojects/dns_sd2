#[derive(Debug, Default)]
pub struct Service {
    _name: String,
    _txt_records: Vec<String>,
    _timeout: u64,
    _state: ServiceState,
}

#[derive(Default)]
pub struct Query {
    _name: String,
    _timeout: u64,
}

#[derive(Debug)]
pub enum ServiceState {
    Prelude,
    Probing,
    Announcing,
    Registered,
    ShuttingDown,
}

impl Default for ServiceState {
    fn default() -> Self {
        ServiceState::Probing
    }
}
