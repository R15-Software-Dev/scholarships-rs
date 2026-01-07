#[derive(Debug, Clone, PartialEq)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub file_type: String,
    pub progress: f64,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UploadStatus {
    Idle,
    Success,
    Error,
}
