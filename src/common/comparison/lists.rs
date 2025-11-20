use super::{Comparison, NumberComparison};
use crate::common::ValueType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NumberListComparison {
    /// Sums the values in the list and checks them using the given [`NumberComparison`].
    /// Requires the list to contain only numbers.
    Sum(Box<NumberComparison>),
    /// Checks if the list contains a specific value.
    Contains,
    /// Checks if the list does not contain a specific value.
    NotContains,
}

impl Comparison for NumberListComparison {
    fn evaluate(&self, comp_value: &ValueType, target_value: &ValueType) -> Result<bool, String> {
        match self {
            NumberListComparison::Sum(num_comp) => {
                // Requires the list to contain only numbers. Get this list or throw an error.
                let student_list = match comp_value {
                    ValueType::List(Some(list)) => list,
                    _ => return Err("Expected a list".into()),
                };

                let sum = student_list.iter().try_fold(0.0, |acc, item| match item {
                    ValueType::Number(Some(num_str)) => {
                        let num = num_str.parse::<f64>().unwrap();
                        Ok(acc + num)
                    }
                    _ => Err("Expected a number".into()),
                });

                // If the sum is not a number, return an error.
                let sum = match sum {
                    Ok(sum) => sum,
                    Err(err) => return Err(err),
                };

                // Finally perform the comparison using this sum... we don't have a function for this yet.
                match num_comp.evaluate(&ValueType::Number(Some(sum.to_string())), target_value) {
                    Ok(result) => Ok(result),
                    Err(err) => Err(err),
                }
            }
            NumberListComparison::Contains => {
                // Get list and check if it contains the target value.
                // This will check the type and the value, in that order. If the type does not match,
                // this will return false.
                let list = match comp_value {
                    ValueType::List(Some(vec)) => vec,
                    _ => return Err("Expected a list".into()),
                };

                // Check if the list contains the target value.
                Ok(list.contains(target_value))
            }
            _ => Err("Not implemented".into()),
        }
    }
}

/// Represents all available comparisons for a text list. All comparisons are comparable to a
/// single value.
/// 
/// For example, using `TextListComparison::Contains` checks if a specific value is contained
/// in the list.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TextListComparison {
    /// Checks if the list contains the target value.
    Contains,
    /// Checks if the list does not contain the target value.
    NotContains,
    /// Checks if the list is empty.
    IsEmpty,
    /// Checks if the list is not empty.
    IsNotEmpty,
}

impl Comparison for TextListComparison {
    fn evaluate(&self, comp_value: &ValueType, target_value: &ValueType) -> Result<bool, String> {
        match self {
            TextListComparison::Contains => {
                let list_opt = match comp_value {
                    ValueType::List(list) => list,
                    _ => return Err("Expected a list".into()),
                };

                // If the list exists, check if it contains the target value.
                // Otherwise, return false. Note that this is not a strongly typed comparison - if
                // the list contains multiple different types, no error is thrown.
                let check = match list_opt {
                    Some(list) => list.contains(target_value),
                    None => false,
                };

                Ok(check)
            }
            TextListComparison::NotContains => {
                let list_opt = match comp_value {
                    ValueType::List(list) => list,
                    _ => return Err("Expected a list".into()),
                };

                // If the list exists, check if it does not contain the target value.
                // Otherwise, return true.
                let check = match list_opt {
                    Some(list) => !list.contains(target_value),
                    None => true,
                };

                Ok(check)
            }
            TextListComparison::IsEmpty => {
                let list_opt = match comp_value {
                    ValueType::List(list) => list,
                    _ => return Err("Expected a list".into()),
                };

                // If the list exists, check if it is empty.
                // Otherwise, return false.
                let check = match list_opt {
                    Some(list) => list.is_empty(),
                    None => true,
                };

                Ok(check)
            }
            TextListComparison::IsNotEmpty => {
                let list_opt = match comp_value {
                    ValueType::List(list) => list,
                    _ => return Err("Expected a list".into()),
                };

                // If the list exists, check if it is not empty.
                // Otherwise, return true.
                let check = match list_opt {
                    Some(list) => !list.is_empty(),
                    None => false,
                };

                Ok(check)
            }
        }
    }
}

/// For right now, nested lists should be flattened into a list of a single type before they
/// are compared. The plan is to create a more robust framework that eventually be able to
/// query specific lists and perform comparisons on them.
/// 
/// For example:
/// ```
/// NestedListComparison::FlattenToTextList(Box::new(TextListComparison::Contains));
/// ```
/// 
/// This will flatten a nested list into a single list of `ValueType::String` enums and then
/// will check if the list contains the indicated target value.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NestedListComparison {
    /// Flattens a nested list into a single text list, then performs the given comparison.
    FlattenToTextList(Box<TextListComparison>),
}

impl Comparison for NestedListComparison {
    fn evaluate(&self, comp_value: &ValueType, target_value: &ValueType) -> Result<bool, String> {
        match self {
            NestedListComparison::FlattenToTextList(t) => {
                // Check the comp_value for a series of lists. We'll then iterate over each list and flatten it to a text list.
                // The end type should be a ValueType::List(Option<Vec<T>>), where T is ValueType::String.
                let list = match comp_value {
                    ValueType::List(Some(list)) => list,
                    _ => return Err("Expected a list of values".to_owned()),
                };

                let mut out: Vec<ValueType> = Vec::new();

                for item in list {
                    // Every item should be another list.
                    let sub_list = match item {
                        ValueType::List(Some(sub_list)) => sub_list,
                        _ => return Err("Expected a list of lists".to_owned()),
                    };

                    for sub_item in sub_list {
                        match sub_item {
                            s @ ValueType::String(_) => out.push(s.clone()),
                            _ => return Err("Expected a list of strings".to_owned()),
                        }
                    }
                }

                t.evaluate(&ValueType::List(Some(out)), target_value)
            }
        }
    }
}

/// Lists of maps must be flattened to a list of some type before they can be compared.
/// Flattening maps simply extracts the values from a specific key from all maps in the list.
/// This makes comparison much simpler to implement and understand.
///
/// All flattening operations also contain a `String` key (the map's member to flatten)
/// and some `Comparison` enum that matches the type of the flattened values. Flattening will
/// fail if any single map in the list does not contain the specified key or if the value
/// is not of the expected type.
///
/// For example, to flatten a list of maps by using the `name` key, we would use the following
/// structure:
/// ```
/// MapListComparison::FlattenToTextList("name".to_owned(), Box::new(TextListComparison::Equal));
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MapListComparison {
    /// Flattens the map list to a text list and then performs the given comparison.
    FlattenToTextList(String, Box<TextListComparison>),
    /// Flattens the map list to a number list and then performs the given comparison.
    FlattenToNumberList(String, Box<NumberListComparison>),
    /// Flattens the map list to a nested list and then performs the given comparison.
    FlattenToNestedList(String, Box<NestedListComparison>),
}

impl Comparison for MapListComparison {
    fn evaluate(&self, comp_value: &ValueType, target_value: &ValueType) -> Result<bool, String> {
        // Define base list - this must be present otherwise we cannot proceed.
        let list = match comp_value {
            ValueType::List(Some(list)) => list,
            ValueType::List(None) => &Vec::new(),
            _ => return Err("Expected a list".to_owned()),
        };

        match self {
            MapListComparison::FlattenToTextList(key, comp) => {
                let mut out = Vec::new();

                for item in list {
                    let map = match item {
                        ValueType::Map(Some(map)) => map,
                        _ => return Err("Expected a map".to_string()),
                    };

                    // Ignore any non-string values.
                    if let Some(text @ ValueType::String(_)) = map.get(key) {
                        out.push(text.clone());
                    }
                }

                let flattened = ValueType::List(Some(out));
                comp.evaluate(&flattened, target_value)
            }
            MapListComparison::FlattenToNumberList(key, comp) => {
                let mut out = Vec::new();
                for item in list {
                    let map = match item {
                        ValueType::Map(Some(map)) => map,
                        _ => return Err("Expected a map".to_string()),
                    };

                    // Ignore any non-number values.
                    if let Some(number @ ValueType::Number(_)) = map.get(key) {
                        out.push(number.clone());
                    }
                }

                let flattened = ValueType::List(Some(out));
                comp.evaluate(&flattened, target_value)
            }
            MapListComparison::FlattenToNestedList(key, comp) => {
                let mut out = Vec::new();
                for item in list {
                    let map = match item {
                        ValueType::Map(Some(map)) => map,
                        _ => return Err("Expected a map".to_string()),
                    };

                    // Ignore any non-list values.
                    if let Some(list @ ValueType::List(_)) = map.get(key) {
                        out.push(list.clone());
                    }
                }

                let flattened = ValueType::List(Some(out));
                comp.evaluate(&flattened, target_value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_utils::{create_map_list_helper};
    use crate::common::{ComparisonType};
    use crate::common::comparison::test_utils::{create_empty_lists, create_number_list, create_text_list};

    #[test]
    fn map_list_flatten_to_text_list() {
        let map_list = create_map_list_helper();

        // Define our comparison, paying close attention to the target values.
        let first_name_comp = ComparisonType::MapList(MapListComparison::FlattenToTextList(
            "first_name".to_string(),
            Box::new(TextListComparison::Contains),
        ));

        let last_name_comp = ComparisonType::MapList(MapListComparison::FlattenToTextList(
            "last_name".to_string(),
            Box::new(TextListComparison::Contains),
        ));

        let gender_comp = ComparisonType::MapList(MapListComparison::FlattenToTextList(
            "gender".to_string(),
            Box::new(TextListComparison::Contains),
        ));

        let first_name_result =
            first_name_comp.evaluate(&map_list, &ValueType::String(Some("Jane".to_string())));
        let last_name_result =
            last_name_comp.evaluate(&map_list, &ValueType::String(Some("Fails".to_string())));
        let gender_result =
            gender_comp.evaluate(&map_list, &ValueType::String(Some("Female".to_string())));

        // We have one map that contains the target value (map_two.first_name)
        assert_eq!(first_name_result.unwrap(), true);
        assert_eq!(last_name_result.unwrap(), false);
        assert_eq!(gender_result.unwrap(), true);
    }

    #[test]
    fn map_list_flatten_to_number_list() {
        let map_list = create_map_list_helper();

        let sum_comparison = ComparisonType::MapList(MapListComparison::FlattenToNumberList(
            "sat_score".to_string(),
            Box::new(NumberListComparison::Sum(Box::new(NumberComparison::Equal))),
        ));

        let result = sum_comparison.evaluate(&map_list, &ValueType::Number(Some(2200.to_string())));
        let failed = sum_comparison.evaluate(&map_list, &ValueType::Number(Some(2201.to_string())));

        assert_eq!(result.unwrap(), true);
        assert_eq!(failed.unwrap(), false);
    }

    #[test]
    fn map_list_flatten_to_nested_list() {
        let map_list = create_map_list_helper();

        let comparison = ComparisonType::MapList(MapListComparison::FlattenToNestedList(
            "grades_participated".to_string(),
            Box::new(NestedListComparison::FlattenToTextList(Box::new(
                TextListComparison::Contains,
            ))),
        ));

        let result = comparison.evaluate(&map_list, &ValueType::String(Some("9".to_string())));
        let failed = comparison.evaluate(&map_list, &ValueType::String(Some("13".to_string())));

        assert_eq!(result.unwrap(), true);
        assert_eq!(failed.unwrap(), false);
    }

    #[test]
    fn number_list_sum_comparison() {
        let list = create_number_list();

        let comparison = ComparisonType::NumberList(NumberListComparison::Sum(Box::new(
            NumberComparison::Equal,
        )));

        let result = comparison.evaluate(&list, &ValueType::Number(Some(15.to_string())));

        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn number_list_contains_all_types() {
        let num_list = create_number_list();

        let string_list = ValueType::List(Some(vec![
            ValueType::String(Some("a".to_string())),
            ValueType::String(Some("b".to_string())),
            ValueType::String(Some("c".to_string())),
            ValueType::String(Some("d".to_string())),
            ValueType::String(Some("e".to_string())),
        ]));

        let mixed_list = ValueType::List(Some(vec![
            ValueType::Number(Some(1.to_string())),
            ValueType::String(Some("b".to_string())),
            ValueType::Number(Some(3.to_string())),
            ValueType::String(Some("d".to_string())),
            ValueType::Number(Some(5.to_string())),
        ]));

        let comparison = ComparisonType::NumberList(NumberListComparison::Contains);

        let num_result = comparison.evaluate(&num_list, &ValueType::Number(Some(3.to_string())));
        let string_result =
            comparison.evaluate(&string_list, &ValueType::String(Some("a".to_string())));
        let mixed_result =
            comparison.evaluate(&mixed_list, &ValueType::String(Some("b".to_string())));

        let incorrect_type =
            comparison.evaluate(&string_list, &ValueType::Number(Some(1.to_string())));
        let incorrect_value =
            comparison.evaluate(&string_list, &ValueType::String(Some("f".to_string())));
        let mixed_incorrect_type =
            comparison.evaluate(&string_list, &ValueType::String(Some("1".to_string())));

        assert_eq!(num_result.unwrap(), true);
        assert_eq!(string_result.unwrap(), true);
        assert_eq!(mixed_result.unwrap(), true);
        assert_eq!(incorrect_type.unwrap(), false);
        assert_eq!(incorrect_value.unwrap(), false);
        assert_eq!(mixed_incorrect_type.unwrap(), false);
    }
    
    #[test]
    fn text_list_contains() {
        let (none_list, empty_list) = create_empty_lists();
        let num_list = create_number_list();
        let text_list = create_text_list();
        
        // Define comparisons here
        let comparison = ComparisonType::TextList(TextListComparison::Contains);
        let target_value = ValueType::String(Some("one".to_string()));
        
        let none_result = comparison.evaluate(&none_list, &target_value);
        let empty_result = comparison.evaluate(&empty_list, &target_value);
        let num_result = comparison.evaluate(&num_list, &target_value);
        let text_list = comparison.evaluate(&text_list, &target_value);
        
        assert_eq!(none_result.unwrap(), false);
        assert_eq!(empty_result.unwrap(), false);
        assert_eq!(num_result.unwrap(), false);
        assert_eq!(text_list.unwrap(), true);
    }
    
    #[test]
    fn text_list_empty() {
        let none_list = ValueType::List(None);
        let empty_list = ValueType::List(Some(vec![]));
        let long_list = ValueType::List(Some(vec![
            ValueType::String(None),
        ]));
        
        let empty_comp = ComparisonType::TextList(TextListComparison::IsEmpty);
        let not_empty_comp = ComparisonType::TextList(TextListComparison::IsNotEmpty);
        
        // This comparison does not use a target value, as it should just check if the list
        // either doesn't exist or is empty.
        let mut result = empty_comp.evaluate(&empty_list, &ValueType::List(None));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
        
        result = empty_comp.evaluate(&none_list, &ValueType::List(None));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
        
        result = empty_comp.evaluate(&long_list, &ValueType::List(None));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
        
        result = not_empty_comp.evaluate(&empty_list, &ValueType::List(None));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
        
        result = not_empty_comp.evaluate(&none_list, &ValueType::List(None));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);
        
        result = not_empty_comp.evaluate(&long_list, &ValueType::List(None));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
