use crate::common::ValueType;

pub trait Comparison {
    fn evaluate(&self, comp_value: &ValueType, target_value: &ValueType) -> Result<bool, String>;
}
