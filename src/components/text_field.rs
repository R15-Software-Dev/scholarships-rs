use crate::common::ValueType;
use leptos::prelude::*;
use std::collections::HashMap;
use leptos::html::Input;
use leptos::logging::debug_log;
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
    /// A text field that accepts the specified email domains.
    /// Values should be given as the string following the '@' in an address, or '*' for any.
    ///
    /// Example usage:
    /// ```
    /// TextFieldType::Email(vec!["gmail.com".to_string(), "customdomain.org".to_string()]);
    /// TextFieldType::Email(vec!["*".to_string()]);
    /// ```
    Email(Vec<String>)
}

/// Entry point for validating a text field.
fn validate(required: bool, value: &str, input_type: &TextFieldType) -> ValidationState {
    if required && value.is_empty() {
        ValidationState::Invalid("This field is required.".to_string())
    } else if !required && value.is_empty() {
        ValidationState::Valid
    } else {
        match input_type {
            TextFieldType::Email(domains) => validate_email(value, domains),
            TextFieldType::Number => validate_number(value),
            TextFieldType::Text => ValidationState::Valid
        }
    }
}

/// Validates an email address.
fn validate_email(input: &str, valid_domains: &Vec<String>) -> ValidationState {
    // Check for wildcard domain. We won't check domains if it's present.
    let wild = valid_domains.contains(&"*".to_string());

    input.split_once('@')
        .and_then(|(_, domain)| {
            (wild || valid_domains.contains(&domain.to_string()))
                .then(|| ValidationState::Valid)
                .or_else(|| Some(ValidationState::Invalid(
                    format!("Email address must match one of the following: {}", valid_domains.join(", "))
                )))
        }).unwrap_or(ValidationState::Invalid("Invalid email address.".to_string()))
}

/// Validates a number.
fn validate_number(input: &str) -> ValidationState {
    if input.parse::<f32>().is_ok() {
        ValidationState::Valid
    } else {
        ValidationState::Invalid("Value is not a valid number.".to_string())
    }
}

#[component]
pub fn OutlinedTextField(
    #[prop(optional, into)] placeholder: String,
    #[prop(into)] data_member: String,
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop(optional, into)] input_type: Signal<TextFieldType>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] required: Signal<bool>
) -> impl IntoView {
    let input_ref = NodeRef::<Input>::new();
    let data_member = StoredValue::new(data_member);

    // Register this input's validation signal.
    let validator_context = use_validation_context()
        .expect("FormValidSignal was not found");
    
    let display_value = RwSignal::new(
        data_map.get_untracked()
            .get(&data_member.get_value())
            .map(|v| v.to_string())
            .unwrap_or_default()
    );

    let dirty = RwSignal::new(false);
    let error = Signal::derive(move || 
        validate(required.get(), &display_value.get(), &input_type.get())
    );
    let show_errors = Signal::derive(move || {
        dirty.get() && matches!(error.get(), ValidationState::Invalid(_))
    });

    let validator = RwSignal::new(InputState::new(data_member.get_value(), error.clone(), dirty.clone()));

    validator_context.validators.update(|list| list.push(validator));

    on_cleanup(move || {
        validator_context.validators.update(|list| {
            list.retain(|v| *v.get_untracked().input_name != *validator.get_untracked().input_name)
        });
    });

    let on_blur = move |_| {
        // Check validity of input's current value. The value will be updated by on_input,
        // and it should be a raw input (not parsed into a ValueType).
        dirty.set(true);
    };

    // Syncs data from the map into the display_value, only when the map is actually changed.
    Effect::new(move || {
        let new_value = data_map.get()
            .get(&data_member.get_value())
            .map(|v| v.to_string())
            .unwrap_or_default();
        
        if new_value != display_value.get_untracked() {
            debug_log!("Found value from map: {}", new_value);
            display_value.set(new_value);
        }
    });
    
    // This function parses the input into the correct type. It only accepts numbers or strings as
    // valid types, and parses them accordingly.
    let on_input = move |e| {
        let to_parse = event_target_value(&e);
        debug_log!("Setting value to display_value: {}", to_parse);
        display_value.set(to_parse.clone());
        
        // Push the new value to the data map. This happens regardless of whether the value parses
        // or not.
        data_map.update(|map| {
            let into_map = match input_type.get() {
                TextFieldType::Number => ValueType::Number(Some(to_parse)),
                _ => ValueType::String(Some(to_parse))
            };

            map.insert(data_member.get_value(), into_map);
        });
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
                    r#type="text"
                    disabled=disabled
                    placeholder=placeholder
                    prop:name=name
                    prop:value=display_value
                    on:input=on_input
                    on:blur=on_blur
                />
                <Show when=move || show_errors.get()>
                    <div class="text-red-600 text-sm mr-1.5 ml-1.5">
                        {move || {
                            match error.get() {
                                ValidationState::Invalid(msg) => msg,
                                _ => "There is no error - should not see this message.".to_string(),
                            }
                        }}
                    </div>
                </Show>
            </label>
        </div>
    }
}
