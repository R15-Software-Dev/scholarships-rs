use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateInfo {
    #[serde()]
    pub(crate) title: String,
    #[serde(rename = "date")]
    pub(crate) date: DateRange,
    #[serde(rename = "desc")]
    pub(crate) description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateRange {
    Single(String),
    Range(String, String),
}
