#[derive(Debug, Default)]
pub struct Service {
    pub name: String,
    pub txt_records: Vec<String>,
    pub timeout: u64,
    pub state: ServiceState,
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
        ServiceState::Prelude
    }
}
