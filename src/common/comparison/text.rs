use super::Comparison;
use crate::common::ValueType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextComparison {
    Matches,
    NotMatches,
    Contains,
    NotContains,
}

impl Comparison for TextComparison {
    fn evaluate(&self, comp_value: &ValueType, target_value: &ValueType) -> Result<bool, String> {
        // All text comparisons require that the comp.member points to a Some(string) in the student_info,
        // and that the target_value is also a Some(string).
        let student_value = match comp_value {
            ValueType::String(Some(value)) => value,
            _ => return Err("Student value is not a string".into()),
        };

        let target_value = match target_value {
            ValueType::String(Some(value)) => value,
            _ => return Err("Target value is not a string".into()),
        };

        let matched = match self {
            TextComparison::Matches => student_value == target_value,
            TextComparison::NotMatches => student_value != target_value,
            TextComparison::Contains => student_value.contains(target_value),
            TextComparison::NotContains => !student_value.contains(target_value),
        };

        Ok(matched)
    }
}
