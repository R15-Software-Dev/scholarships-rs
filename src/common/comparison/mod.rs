mod base;
mod lists;
mod number;
mod text;
mod traits;

pub use self::{base::*, lists::*, number::*, text::*, traits::*};

#[cfg(test)]
mod test_utils {
    use crate::common::{ExpandableInfo, ValueType};
    use std::collections::HashMap;

    pub fn create_student_data() -> ExpandableInfo {
        let map = create_map_helper(
            vec![
                "first_name".to_string(),
                "last_name".to_string(),
                "gender".to_string(),
                "sat_score".to_string(),
                "athletic_participation".to_string(),
                "community_involvement".to_string(),
            ],
            vec![
                ValueType::String(Some("John".to_string())),
                ValueType::String(Some("Doe".to_string())),
                ValueType::String(Some("Male".to_string())),
                ValueType::Number(Some(1190.to_string())),
                ValueType::Map(Some(create_map_helper(
                    vec![
                        "sport_name".to_string(),
                        "grades_participated".to_string(),
                        "special_achievement".to_string(),
                    ],
                    vec![
                        ValueType::String(Some("Football".to_string())),
                        ValueType::List(Some(vec![
                            ValueType::String(Some(9.to_string())),
                            ValueType::String(Some(10.to_string())),
                        ])),
                        ValueType::String(Some("N/A".to_string())),
                    ],
                ))),
                ValueType::Map(Some(create_map_helper(
                    vec!["activity_name".to_string(), "total_hours".to_string()],
                    vec![
                        ValueType::String(Some("Raking".to_string())),
                        ValueType::Number(Some(30.to_string())),
                    ],
                ))),
            ],
        );

        let mut info = ExpandableInfo::new("test");
        info.data = map;

        info
    }

    pub fn create_map_helper(
        keys: Vec<String>,
        values: Vec<ValueType>,
    ) -> HashMap<String, ValueType> {
        keys.iter()
            .cloned()
            .zip(values.iter().cloned())
            .collect::<HashMap<String, ValueType>>()
    }

    pub fn create_map_list_helper() -> ValueType {
        let map_one = create_map_helper(
            vec![
                "first_name".to_string(),
                "last_name".to_string(),
                "gender".to_string(),
                "sat_score".to_string(),
                "grades_participated".to_string(),
            ],
            vec![
                ValueType::String(Some("John".to_string())),
                ValueType::String(Some("Doe".to_string())),
                ValueType::String(Some("Male".to_string())),
                ValueType::Number(Some(1200.to_string())),
                ValueType::List(Some(vec![
                    ValueType::String(Some("9".to_string())),
                    ValueType::String(Some("10".to_string())),
                ])),
            ],
        );

        let map_two = create_map_helper(
            vec![
                "first_name".to_string(),
                "last_name".to_string(),
                "gender".to_string(),
                "sat_score".to_string(),
                "grades_participated".to_string(),
            ],
            vec![
                ValueType::String(Some("Jane".to_string())),
                ValueType::String(Some("Doe".to_string())),
                ValueType::String(Some("Female".to_string())),
                ValueType::Number(Some(1000.to_string())),
                ValueType::List(Some(vec![
                    ValueType::String(Some("11".to_string())),
                    ValueType::String(Some("12".to_string())),
                ])),
            ],
        );

        ValueType::List(Some(vec![
            ValueType::Map(Some(map_one)),
            ValueType::Map(Some(map_two)),
        ]))
    }

    pub fn create_empty_lists() -> (ValueType, ValueType) {
        let none_list = ValueType::List(None);
        let empty_list = ValueType::List(Some(vec![]));

        (none_list, empty_list)
    }

    pub fn create_number_list() -> ValueType {
        ValueType::List(Some(vec![
            ValueType::Number(Some(1.to_string())),
            ValueType::Number(Some(2.to_string())),
            ValueType::Number(Some(3.to_string())),
            ValueType::Number(Some(4.to_string())),
            ValueType::Number(Some(5.to_string())),
        ]))
    }

    pub fn create_text_list() -> ValueType {
        ValueType::List(Some(vec![
            ValueType::String(Some("one".to_string())),
            ValueType::String(Some("two".to_string())),
            ValueType::String(Some("three".to_string())),
            ValueType::String(Some("four".to_string())),
            ValueType::String(Some("five".to_string())),
        ]))
    }
}
