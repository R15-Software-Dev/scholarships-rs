use macros::Reactive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Reactive)]
pub struct StudentInfo {
    pub first_name: String,
    pub last_name: String,
    pub math_sat_score: i32,
}
