use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentInfo {
    pub first_name: String,
    pub last_name: String,
}
