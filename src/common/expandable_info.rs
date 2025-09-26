use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use macros::Reactive;
use crate::components::ValueType;

/// This struct allows for completely open-ended data storage.
/// 
/// The `HashMap` allows the application to store an arbitrary number of values which are accessible
/// using a `String` as a key, allowing for simple shifting of stored values. All information is
/// stored using the `ValueType` enum, which provides some simple methods to verify the type of the
/// stored value. It *must* have a main key member name so that a database will know how to store
/// the data.
/// 
/// In the future, this should also be optimized to store properly within a DynamoDB table. As of
/// now, the table will simply create a single massive `Map` field that contains all the stored
/// information in the `HashMap`. We'd prefer to be able to get single values if possible,
/// especially for interoperability with other applications, should they be created.
#[derive(Debug, Clone, Serialize, Deserialize, Reactive)]
pub struct ExpandableInfo {
    /// The main key member. Required and used to get/put the information in a database.
    pub subject: String,
    /// The main data store. Contains all information that this struct needs to hold.
    #[serde(default, flatten)]
    pub data: HashMap<String, ValueType>,
}

impl ExpandableInfo {
    pub fn new(key: String) -> Self {
        Self {
            subject: key,
            data: HashMap::new(),
        }
    }
}
