use super::{
    Comparison, MapListComparison, NestedListComparison, NumberComparison, NumberListComparison,
    TextComparison, TextListComparison,
};
use crate::common::ValueType;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComparisonData {
    /// Each comparison will have a unique ID.
    pub id: String,
    /// The member from the Student structure to match against.
    pub member: String,
    /// The comparison to perform.
    pub comparison: ComparisonType,
    /// The value to match against.
    pub target_value: ValueType,
    /// The display category for this comparison. Affects where this comparison
    /// will be displayed in the provider side of the application.
    pub category: String,
    /// The text that will be displayed on the provider side.
    pub display_text: String,
}

impl ComparisonData {
    #[allow(unused)]
    pub fn new(
        id: impl Into<String>,
        member: impl Into<String>,
        comparison: ComparisonType,
        target_value: ValueType,
        category: impl Into<String>,
        display_text: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            member: member.into(),
            comparison,
            target_value,
            category: category.into(),
            display_text: display_text.into(),
        }
    }

    pub fn compare(&self, student_data: &HashMap<String, ValueType>) -> Result<bool, String> {
        student_data.get(&self.member)
            .ok_or_else(||
                format!("Couldn't find member {:?}, current student data: {:?}", self.member, student_data)
            )
            .and_then(|value| self.comparison.evaluate(value, &self.target_value))
            .or_else(|err| Err(err.to_string()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ComparisonType {
    Number(NumberComparison),
    Text(TextComparison),
    TextList(TextListComparison),
    NumberList(NumberListComparison),
    MapList(MapListComparison),
    NestedList(NestedListComparison),
}

impl Comparison for ComparisonType {
    fn evaluate(&self, comp_value: &ValueType, target_value: &ValueType) -> Result<bool, String> {
        match self {
            ComparisonType::Number(n) => n.evaluate(comp_value, target_value),
            ComparisonType::Text(t) => t.evaluate(comp_value, target_value),
            ComparisonType::NumberList(l) => l.evaluate(comp_value, target_value),
            ComparisonType::MapList(m) => m.evaluate(comp_value, target_value),
            ComparisonType::TextList(tl) => tl.evaluate(comp_value, target_value),
            ComparisonType::NestedList(nl) => nl.evaluate(comp_value, target_value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::comparison::test_utils::create_student_data;

    #[test]
    fn test_student_data() {
        let student_data = create_student_data();

        // Should succeed every time.
        let text_comp_success = ComparisonData {
            id: "first_name_success".to_string(),
            member: "first_name".to_string(),
            comparison: ComparisonType::Text(TextComparison::Matches),
            target_value: ValueType::String(Some("John".to_string())),
            category: "Basic Checks".to_string(),
            display_text: "First Name is John".to_string(),
        };

        // Uses an incorrect target_value type.
        let text_comp_fail = ComparisonData {
            id: "first_name_fail".to_string(),
            member: "first_name".to_string(),
            comparison: ComparisonType::Text(TextComparison::Matches),
            target_value: ValueType::Number(Some("Jane".to_string())),
            category: "Basic Checks".to_string(),
            display_text: "First Name is Jane".to_string(),
        };

        let mut result = text_comp_success.compare(&student_data.data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        result = text_comp_fail.compare(&student_data.data);
        assert!(result.is_err());
    }
}
