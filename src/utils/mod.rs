use crate::common::{ExpandableInfo, ValueType};
use std::ops::ControlFlow;

pub fn get_number_list(student_info: &ExpandableInfo, member: &String) -> Result<Vec<f64>, String> {
    match student_info.data.get(member) {
        Some(ValueType::List(Some(list))) => {
            let temp = list.iter().try_fold(vec![], |mut list_acc, item| {
                if let ValueType::Number(Some(number)) = item {
                    list_acc.push(number.parse::<f64>().unwrap());
                    ControlFlow::Continue(list_acc)
                } else {
                    ControlFlow::Break(list_acc)
                }
            });

            match temp {
                ControlFlow::Continue(list) => Ok(list),
                ControlFlow::Break(_) => return Err("List contains non-number values".to_owned()),
            }
        }
        _ => return Err("Couldn't find the list".to_owned()),
    }
}
