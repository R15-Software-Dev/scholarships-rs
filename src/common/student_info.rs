use macros::Reactive;
use serde::{Deserialize, Serialize};
use std::default::Default;

/// Represents information about a student.
#[derive(Serialize, Deserialize, Debug, Clone, Reactive, Default)]
pub struct StudentInfo {
    /// The email address of the student. This was used in the previous version of the database,
    /// and should now be deprecated.
    ///
    /// This is the primary key of the table, and should not have a default value.
    pub Email: String,
    /// The student's first name.
    #[serde(default)]
    pub first_name: String,
    /// The student's last name.
    #[serde(default)]
    pub last_name: String,
    /// The student's highest math score on the SAT.
    #[serde(default)]
    pub math_sat: i32,
    /// The student's desired contact email.
    #[serde(default)]
    pub contact_email: String,
}
