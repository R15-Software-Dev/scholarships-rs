use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// Defines a set of possible input types. They allow for the type and value of every data member
/// to be stored within a single `HashMap` or `Vec`. This means that even though all the data is
/// created as a single type, the values are still able to be type safe.
///
/// A note: other structs cannot be stored in this enum, however it was designed to work with
/// a `HashMap` of more `ValueType` enums. There are numerous ways to convert a struct into a
/// `HashMap`, specifically storing all the struct's fields as `String` keys in the map.
#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum ValueType {
    /// Indicates a `String` value.
    String(Option<String>),
    /// Indicates an `i32` value.
    Number(Option<String>),
    /// Indicates a `Vec<T>`, where `T` can be any other `ValueType`.
    List(Option<Vec<ValueType>>),
    /// Indicates a `HashMap<String, ValueType>`.
    Map(Option<HashMap<String, ValueType>>),
}

impl ValueType {
    /// Returns true if this is a `String`, otherwise false.
    pub fn is_string(&self) -> bool {
        if let ValueType::String(_) = self {
            true
        } else {
            false
        }
    }

    /// Attempts to get this value as a `String` type. This is different from the `to_string` method.
    pub fn as_string(&self) -> Result<Option<String>, Self> {
        if let ValueType::String(v) = self {
            Ok(v.clone())
        } else {
            Err(self.clone())
        }
    }

    /// Returns true if this is a `Number`, otherwise false.
    pub fn is_number(&self) -> bool {
        if let ValueType::Number(_) = self {
            true
        } else {
            false
        }
    }

    /// Attempts to get this value as an `i32`.
    pub fn as_number(&self) -> Result<Option<String>, Self> {
        if let ValueType::Number(v) = self {
            Ok(v.clone())
        } else {
            Err(self.clone())
        }
    }

    /// Returns true if this is a `List`, otherwise false.
    pub fn is_list(&self) -> bool {
        if let ValueType::List(_) = self {
            true
        } else {
            false
        }
    }

    /// Attempts to get this value as a `Vec<ValueType>`. Each of the values inside the list must
    /// also be converted later, as these values are any of the possible types in the `ValueType`.
    pub fn as_list(&self) -> Result<Option<Vec<ValueType>>, Self> {
        if let ValueType::List(v) = self {
            Ok(v.clone())
        } else {
            Err(self.clone())
        }
    }

    /// Returns true if this is a `Map`, otherwise false.
    pub fn is_map(&self) -> bool {
        if let ValueType::Map(_) = self {
            true
        } else {
            false
        }
    }

    /// Attempts to get this value as a `HashMap<String, ValueType>`.
    pub fn as_map(&self) -> Result<Option<HashMap<String, ValueType>>, Self> {
        if let ValueType::Map(v) = self {
            Ok(v.clone())
        } else {
            Err(self.clone())
        }
    }
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::String(Some(String::new()))
    }
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ValueType::String(s) => s.clone().unwrap_or("".to_owned()),
            ValueType::Number(n) => n.clone().unwrap_or("".to_owned()),
            ValueType::List(l) => format!("{l:?}"),
            ValueType::Map(m) => format!("{m:?}"),
        };
        write!(f, "{}", str)
    }
}
