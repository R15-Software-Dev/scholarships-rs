use macros::Reactive;
use serde::{Deserialize, Serialize};
use std::default::Default;

/// Represents information about a student.
#[derive(Serialize, Deserialize, Debug, Clone, Reactive, Default)]
pub struct StudentInfo {
    /// The email address of the student. This was used in the previous version of the database,
    /// and should now be deprecated.
    pub Email: String,
    /// The student's first name.
    #[serde(default)]
    pub studentFirstName: String,
    /// The student's last name.
    #[serde(default)]
    pub studentLastName: String,
    /// The student's highest math score on the SAT.
    #[serde(default)]
    pub mathScoreSAT: i32,
    /// The student's desired contact email.
    #[serde(default)]
    pub studentEmail: String,
}
