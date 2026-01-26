#[derive(Debug, Clone)]
pub struct TabInfo {
    pub text: String,
    pub path: String,
}

impl TabInfo {
    pub fn new(text: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            path: path.into(),
        }
    }
}