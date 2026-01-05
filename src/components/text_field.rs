use crate::common::ValueType;
use leptos::prelude::*;
use std::collections::HashMap;
use leptos::html::Input;
use crate::components::{use_validation_context, InputState};
use crate::components::validated_form::ValidationState;

/// Determines the available types of text fields.
#[derive(Default, Debug, Clone)]
pub enum TextFieldType {
    /// A text field that accepts any string.
    #[default]
    Text,
    /// A text field that accepts only numbers.
    Number,
    /// A text field that accepts only Region 15 email addresses.
    /// TODO Accept more than just Region 15 addresses.
    Email
}

/// Entry point for validating a text field.
fn validate(required: bool, value: &str, input_type: &TextFieldType) -> ValidationState {
    if required && value.is_empty() {
        ValidationState::Invalid("This field is required.".to_string())
    } else {
        match input_type {
            TextFieldType::Email => validate_email(value),
            TextFieldType::Number => validate_number(value),
            TextFieldType::Text => ValidationState::Valid
        }
    }
}

/// Validates an email address.
fn validate_email(input: &str) -> ValidationState {
    // TODO Accept more than just Region 15 addresses.
    let pattern = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@region15\.org$").expect("Invalid regex pattern");

    if pattern.is_match(input) {
        ValidationState::Valid
    } else {
        ValidationState::Invalid("Please provide a valid Region 15 email address.".to_string())
    }
}

/// Validates a number.
fn validate_number(input: &str) -> ValidationState {
    if input.parse::<i32>().is_ok() {
        ValidationState::Valid
    } else {
        ValidationState::Invalid("Value is not a valid number.".to_string())
    }
}

#[component]
pub fn OutlinedTextField(
    #[prop(optional, into)] placeholder: String,
    #[prop(into)] data_member: String, // should this be an RwSignal??
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop(optional)] input_type: TextFieldType,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] required: Signal<bool>
) -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();

    // Register this input's validation signal.
    let validator_context = use_validation_context()
        .expect("FormValidSignal was not found");

    let raw_value = RwSignal::new(data_map.get_untracked()
        .get(&data_member)
        .unwrap_or(&ValueType::String(None))
        .to_string());

    let dirty = RwSignal::new(false);
    let error = RwSignal::new(validate(required.get_untracked(), &raw_value.get_untracked(), &input_type));
    let show_errors = Signal::derive(move || {
        dirty.get() && matches!(error.get(), ValidationState::Invalid(_))
    });

    let validator = RwSignal::new(InputState::new(data_member.clone(), error.clone(), dirty.clone()));

    validator_context.validators.update(|list| list.push(validator));

    on_cleanup(move || {
        validator_context.validators.update(|list| {
            list.retain(|v| *v.get_untracked().input_name != *validator.get_untracked().input_name)
        });
    });

    let str_type = match input_type {
        TextFieldType::Number => "number",
        _ => "text"  // This is fine even for emails, as we're doing our own validation.
    };

    let on_blur = move |_| {
        // Check validity of input's current value. The value will be updated by on_input,
        // and it should be a raw input (not parsed into a ValueType).
        dirty.set(true);
    };

    // This function parses the input into the correct type. It only accepts numbers or strings as
    // valid types, and parses them accordingly.
    let on_input = {
        let data_member = data_member.clone();
        move |e| {
            let to_parse = event_target_value(&e);
            raw_value.set(to_parse.clone());
            error.set(validate(required.get(), &to_parse, &input_type));
            let into_map = match input_type {
                TextFieldType::Number => ValueType::Number(Some(to_parse)),
                _ => ValueType::String(Some(to_parse))
            };

            // Edit the data map. Note that this should be uncoupled in the future - we don't want
            // to update the map inside the component definition, but inside the page/form that
            // created this input.
            data_map.update(|map| {
                map.insert(data_member.clone(), into_map);
            });
        }
    };

    view! {
        <div class="flex flex-1">
            <label class="flex flex-col flex-1">
                <span class="block ml-1.5 mb-0 font-bold">{label}</span>
                <input
                    node_ref=input_ref
                    class="border-2 m-1.5 p-1.5 mt-0 rounded-md bg-transparent relative flex-1
                        transition-all duration-150
                        border-red-700 bg-transparent
                        disabled:border-gray-600 disabled:pointer-events-none disabled:bg-gray-600/33"
                    r#type={str_type}
                    disabled={disabled}
                    placeholder={placeholder}
                    prop:name=name
                    prop:value=move || data_map
                        .get()  // HashMap
                        .get(&data_member)  // Option<ValueType>
                        .unwrap_or(&ValueType::String(None))  // ValueType
                        .to_string()
                    on:input=on_input
                    on:blur=on_blur
                />
                <Show when=move || show_errors.get()>
                    <div class="text-red-600 text-sm mr-1.5 ml-1.5">
                        {move || {
                            match error.get() {
                                ValidationState::Invalid(msg) => msg,
                                _ => "There is no error - should not see this message.".to_string()
                            }
                        }}
                    </div>
                </Show>
            </label>
        </div>
    }
}
