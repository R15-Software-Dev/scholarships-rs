use std::collections::HashMap;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SchemaType {
    String,
    Number,
    List,
    Map
}

/// # Schema Node
///
/// This is used to create a schema that is used in the [`ValueDisplay`] component. It stores all
/// required information for the display, specifically where to find the data and how to display it.
/// It covers all `ValueType` variants, however for data storage and DX it uses a `SchemaType`
/// instead.
///
/// Some examples are as follows, using the fluent builder notation:
///  - Displaying a single `ValueType::String` from the data map at the key `example_one`:
///     ```
///     SchemaNode::new(SchemaType::String).header("The example string")
///     ```
///  - Displaying a `ValueType::List` which contains only `ValueType::String` entries:
///     ```
///     SchemaNode::new(SchemaType::List)
///         .header("The example list")
///         .item_template(SchemaNode::new(SchemaType::String).header(""))
///     ```
///  - Displaying a `ValueType::Map` which contains a name and age:
///     ```
///     SchemaType::new(SchemaType::Map)
///         .header("The example map")
///         .child("first_name", SchemaNode::new(SchemaType::String).header("First Name:"))
///         .child("age", SchemaNode::new(SchemaType::Number).header("Age:"))
///     ```
///  - Put it all together: a `ValueType::List` which contains `ValueType::Map` entries, each of
///    which also contains the name and age from the previous example:
///     ```
///     SchemaNode::new(SchemaType::List)
///         .header("The example map list")
///         .item_template(
///             SchemaNode::new(SchemaType::Map)
///                 .header("The example map from earlier")
///                 .child("first_name", SchemaNode::new(SchemaType::String).header("First Name:"))
///                 .child("age", SchemaNode::new(SchemaType::Number).header("Age:"))
///         )
///     ```
///
/// This schema allows for definitive typing and styling for every level of the data display.
///
/// ## Important Notes
///
/// This schema does not currently support display of a single nested member, like a single map
/// within a list. This means that you will have to display any listed or mapped information as a
/// list. Consider this when designing the data.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SchemaNode {
    /// The datatype of the data found using the `data_member` key.
    pub data_type: SchemaType,
    /// The header of this display.
    pub header: String,
    /// When used with a SchemaType::List, this determines the datatype and children for every entry
    /// in the list. Some examples include a list of maps, where we use the `item_template` to define
    /// the SchemaType::Map, and all its children. This is then mapped across all entries in the list,
    /// where any entries that do not match the template are skipped.
    pub item_template: Option<Box<SchemaNode>>,
    /// When used with a SchemaType::Map, this determines what children from the map are displayed
    /// and how. It is ignored with all other SchemaType variants.
    pub children: IndexMap<String, SchemaNode>,
}

impl SchemaNode {
    pub fn new(data_type: SchemaType) -> Self {
        SchemaNode {
            data_type,
            header: "".to_string(),
            item_template: None,
            children: IndexMap::new(),
        }
    }

    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = header.into();
        self
    }

    pub fn child(mut self, data_member: impl Into<String>, node: SchemaNode) -> Self {
        self.children.insert(data_member.into(), node);
        self
    }

    pub fn item_template(mut self, node: SchemaNode) -> Self {
        self.item_template = Some(Box::new(node));
        self
    }
}
