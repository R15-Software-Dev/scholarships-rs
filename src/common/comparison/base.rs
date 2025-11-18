use super::{
    Comparison, MapListComparison, NestedListComparison, NumberComparison, NumberListComparison,
    TextComparison, TextListComparison,
};
use crate::common::{ExpandableInfo, ValueType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn compare(&self, student_data: &ExpandableInfo) -> Result<bool, String> {
        let value = student_data.data.get(&self.member).unwrap();
        self.comparison.evaluate(value, &self.target_value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
    use crate::common::comparison::test_utils::create_student_data;
    use super::*;
    
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
        
        let mut result = text_comp_success.compare(&student_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
        
        result = text_comp_fail.compare(&student_data);
        assert!(result.is_err());
    }
}
