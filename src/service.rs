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

#[derive(Debug, Copy, Clone)]
pub enum ServiceState {
    Prelude,
    WaitForFirstProbe,
    FirstProbe,
    WaitForSecondProbe,
    SecondProbe,
    WaitForAnnouncing,
    FirstAnnouncement,
    WaitForSecondAnnouncement,
    SecondAnnouncement,
    Registered,
    ShuttingDown,
}

impl Default for ServiceState {
    fn default() -> Self {
        ServiceState::Prelude
    }
}
