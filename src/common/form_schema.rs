use serde::{Serialize, Deserialize};

/// Defines a series of options that allow a user to define a form.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InputType{
    /// Represents a text input. Requires a `String` key. The entered value will be returned as a
    /// single `String` value.
    Text,
    /// Represents a number input. Requires a `String` key. The entered value will be returned as a
    /// single `i32` value.
    Number,
    /// Represents a radio input. Requires a `String` key and a series of options. The selected
    /// value will be returned as a single `String` value.
    Radio(Vec<String>),
    /// Represents a checkbox input. Requires a `String` key and a series of options. The selected
    /// value will be returned as a `Vec<String>`.
    Checkbox(Vec<String>)
}
