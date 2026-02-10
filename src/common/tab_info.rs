#[derive(Debug, Clone)]
pub struct TabInfo {
    pub text: String,
    pub path: String,
    pub sub_paths: Vec<SubTabInfo>,
}

impl TabInfo {
    pub fn new(
        text: impl Into<String>,
        path: impl Into<String>,
        sub_paths: Option<Vec<SubTabInfo>>
    ) -> Self {
        Self {
            text: text.into(),
            path: path.into(),
            sub_paths: sub_paths.unwrap_or_default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubTabInfo {
    pub text: String,
    pub path: String,
}

impl SubTabInfo {
    pub fn new(text: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            path: path.into()
        }
    }
}
