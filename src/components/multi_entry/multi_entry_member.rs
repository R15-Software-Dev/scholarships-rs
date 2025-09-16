use serde::{Deserialize, Serialize};
use crate::common::InputType;

/// Contains all information required to display an entry in the MultiEntry component.
///
/// NOTE: This does not define a schema for the data stored by the component, only the data that the
/// component displays as "identifying information."
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MultiEntryMember {
    /// The name of the member, used for display only.
    pub display_name: String,
    /// The actual name of the member, used to get the data from an Entry struct.
    pub member_name: String,
    /// The type of input that should be used to display this member.
    pub input_type: InputType
}

impl MultiEntryMember {
    /// Creates a new MultiEntryMember struct.
    pub fn _new() -> Self {
        Self {
            display_name: String::new(),
            member_name: String::new(),
            input_type: InputType::Text,
        }
    }

    /// Creates a new MultiEntryMember struct with the specified information.
    pub fn _new_with_data(display_name: String, member_name: String, input_type: InputType) -> Self {
        Self {
            display_name,
            member_name,
            input_type
        }
    }

    /// Creates a new MultiEntryMember struct with the specified information. Uses string slices.
    pub fn from_str(display_name: &str, member_name: &str) -> Self {
        Self {
            display_name: display_name.into(),
            member_name: member_name.into(),
            input_type: InputType::Text
        }
    }
}
