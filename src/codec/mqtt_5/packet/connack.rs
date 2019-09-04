#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Connack {
    pub session_present: bool,
    pub code: ConnectReturnCode
}