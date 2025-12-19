#[derive(Debug, Clone)]
pub enum SubmitStatus {
    Idle,
    Sending,
    Success,
    Error(String),
}
