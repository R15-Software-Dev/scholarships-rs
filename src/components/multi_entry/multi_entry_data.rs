use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use macros::Reactive;
use crate::components::ValueType;

/// Stores all data for the `MultiEntry` component.
///
/// All values are stored within the  `HashMap`. These entries only contain the value and
/// the type of the value itself. Each piece of data also has a unique ID, which allows for simpler
/// dynamic rendering.`data`
#[derive(Clone, Serialize, Deserialize, Debug, Reactive, Default)]
pub struct MultiEntryData {
    /// The data's unique ID.
    pub id: Uuid,
    /// The pertinent data for this struct. This may be variable - each `MultiEntry` component
    /// may specify its own schema and way of storing this data. The only limitation will be the
    /// types of data that can be stored - they must be easily parsable by a standard HTML input.
    pub data: HashMap<String, ValueType>,
}

impl MultiEntryData {
    /// Creates a new MultiEntryData struct.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            data: HashMap::new(),
        }
    }
}
