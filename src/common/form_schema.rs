use crate::common::ValueType;
use crate::components::{CheckboxList, OutlinedTextField, RadioList, Row, Select};
use leptos::either::EitherOf5;
use leptos::prelude::RwSignal;
use leptos::{IntoView, view};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Defines a series of options that allow a user to define a form.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum InputType {
    /// Represents a text input. Requires a `String` key, `String` label, and `String` placeholder.
    /// The entered value will be returned as a single `String` value.
    Text(String, String, String),
    /// Represents a number input. Requires a `String` key, `String` label, and a `String` placeholder.
    /// The entered value will be returned as a `String`, and will be parseable into any number type
    /// (i.e. an `i32`, `i64`, etc)
    Number(String, String, String),
    /// Represents a radio input. Requires a `String` key, `String` label, and a series of `String`
    /// options. The selected value will be returned as a `String`.
    Radio(String, String, Vec<String>),
    /// Represents a checkbox input. Requires a `String` key, `String` label, and a series of `String`
    /// options. The selected values will be returned as a `Vec<String>`.
    Checkbox(String, String, Vec<String>),
    /// Represents a dropdown input. Requires a `String` key, `String` label, and a series
    /// of `String` options. The selected value will be returned as a `String`.
    Select(String, String, Vec<String>),
}

impl InputType {
    pub fn into_view(self, data_map: RwSignal<HashMap<String, ValueType>>) -> impl IntoView {
        view! {
            {match self {
                InputType::Text(member, label, placeholder) =>
                    EitherOf5::A(view! {
                        <Row>
                            <OutlinedTextField
                                label = label.clone()
                                placeholder = placeholder.clone()
                                data_member = member.clone()
                                data_map = data_map
                            />
                        </Row>
                    }),
                InputType::Number(member, label, placeholder) =>
                    EitherOf5::B(view! {
                        <OutlinedTextField
                            label = label.clone()
                            placeholder = placeholder.clone()
                            data_member = member.clone()
                            data_map = data_map
                        />
                    }),
                InputType::Checkbox(member, label, options) =>
                    EitherOf5::C(view! {
                        <CheckboxList
                            label = label.clone()
                            items = options.clone()
                            data_member = member.clone()
                            data_map = data_map
                        />
                    }),
                InputType::Radio(member, label, options) =>
                    EitherOf5::D(view! {
                        <RadioList
                            label = label.clone()
                            data_member = member.clone()
                            data_map = data_map
                            items = options.clone()
                        />
                    }),
                InputType::Select(member, label, options) =>
                    EitherOf5::E(view! {
                        <Select
                            label = label.clone()
                            value_list = options.clone()
                            data_member = member.clone()
                            data_map = data_map
                        />
                    })
            }}
        }
    }

    pub fn into_view_rows(self, data_map: RwSignal<HashMap<String, ValueType>>) -> impl IntoView {
        view! {
            <Row>
                {self.into_view(data_map)}
            </Row>
        }
    }
}

/// Shorthand macro for creating `InputType` enums.
///
/// Example usage:
/// ```
/// // To create an InputType::Text
/// let text_input = input!(Text, "member", "label", "placeholder");
///
/// // To create an InputType::Select
/// let select_input = input!(Select, "member", "label", ["option1", "option2"]);
/// ```
#[macro_export]
macro_rules! input {
    ($variant:ident, $key:expr, $label:expr, [$($option:expr),* $(,)?]) => {
        $crate::common::InputType::$variant(
            $key.to_string(),
            $label.to_string(),
            vec![$($option.to_string()), *]
        )
    };

    ($variant:ident, $key:expr, $label:expr, $placeholder:expr) => {
        $crate::common::InputType::$variant(
            $key.to_string(),
            $label.to_string(),
            $placeholder.to_string()
        )
    };
}
