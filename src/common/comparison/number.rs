use super::Comparison;
use crate::common::ValueType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NumberComparison {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl Comparison for NumberComparison {
    fn evaluate(&self, comp_value: &ValueType, target_value: &ValueType) -> Result<bool, String> {
        // The prerequisites for all number comparisons is that the target value is Some(number) and the
        // comparison member points to Some(number) in the student_info.
        let student_value = match comp_value {
            ValueType::Number(Some(num)) => num.parse::<i32>().unwrap(),
            _ => {
                return Err("Student value is not a number".into());
            }
        };

        let target = match target_value {
            ValueType::Number(Some(num)) => num.parse::<i32>().unwrap(),
            _ => {
                return Err("Target value is not a number".into());
            }
        };

        let matched = match self {
            NumberComparison::GreaterThan => student_value > target,
            NumberComparison::LessThan => student_value < target,
            NumberComparison::Equal => student_value == target,
            NumberComparison::NotEqual => student_value != target,
            NumberComparison::GreaterThanOrEqual => student_value >= target,
            NumberComparison::LessThanOrEqual => student_value <= target,
        };

        Ok(matched)
    }
}
