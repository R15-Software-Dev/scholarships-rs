use crate::common::ValueType;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use std::collections::HashMap;
use leptos::html::Input;

#[component]
pub fn OutlinedTextField(
    #[prop(optional, into)] placeholder: String,
    // #[prop()] value: Subfield<TStore, TInner, ValueType>,
    #[prop(into)] data_member: String, // should this be an RwSignal??
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop(optional)] value: RwSignal<ValueType>,
    #[prop(default = "text".to_owned(), into)] input_type: String,
    #[prop(optional)] disabled: RwSignal<bool>,
    #[prop(optional)] error: RwSignal<bool>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] error_text: RwSignal<String>,
) -> impl IntoView {
    let valid = RwSignal::new(true);
    let input_ref = NodeRef::<Input>::new();
    
    // This function parses the input into the correct type. It only accepts numbers or strings as
    // valid types, and parses them accordingly.
    let on_input = {
        let input_type = input_type.clone();
        let data_member = data_member.clone();
        move |e| {
            let to_parse = event_target_value(&e);
            match input_type.as_str() {
                "text" => {
                    // String parse function. It will always parse, but make sure the error
                    // state is set properly anyway.
                    error.set(false);
                    value.set(ValueType::String(Some(to_parse.clone())));
                    data_map.update(|map| {
                        map.insert(data_member.clone(), ValueType::String(Some(to_parse)));
                    });
                }
                "number" => {
                    // Define number parse function.
                    // If the number parses, Some(num), else None, but we want to keep the value that's
                    // in the text field at this point, just set the error.
                    console_log(format!("Parsing {to_parse} to a number").as_str());
                    match to_parse
                        .parse::<i32>()
                        .map(|n| ValueType::Number(Some(n.to_string())))
                    {
                        Ok(value_type) => {
                            error.set(false);
                            value.set(value_type.clone());
                            data_map.update(|map| {
                                map.insert(data_member.clone(), value_type);
                            });
                        }
                        Err(_) => {
                            error.set(true);
                            error_text.set("Value is not a valid number.".to_owned());
                        }
                    };
                }
                "email" => {
                    // Check the validity of the component.
                    if let Some(input) = input_ref.get() {
                        if !input.validity().valid() {
                            error.set(true);
                            if input.validity().pattern_mismatch() {
                                error_text.set("Please provide a valid Region 15 email address.".to_string());
                            }
                        } else {
                            error.set(false);
                            error_text.set("".to_string());
                            value.set(ValueType::String(Some(to_parse.clone())));
                            data_map.update(|map| {
                                map.insert(data_member.clone(), ValueType::String(Some(to_parse)));
                            });
                        }
                    } else {
                        error.set(false);
                        error_text.set("".to_string());
                        value.set(ValueType::String(Some(to_parse.clone())));
                        data_map.update(|map| { 
                            map.insert(data_member.clone(), ValueType::String(Some(to_parse)));
                        });
                    }
                }
                _ => {
                    // Create some custom function that reports failure.
                    let msg = format!("Input {input_type:?} is not a valid type.");
                    console_log(msg.as_str());
                    error.set(true);
                    error_text.set(msg);
                }
            }
        }
    };
    
    let pattern = match input_type.as_str() {
        "email" => ".+@region15\\.org".to_string(),
        _ => String::new()
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
                    r#type={input_type}
                    disabled={disabled}
                    placeholder={placeholder}
                    pattern={pattern}
                    prop:name=name
                    prop:value=move || data_map
                        .get()  // HashMap
                        .get(&data_member)  // Option<ValueType>
                        .unwrap_or(&ValueType::String(None))  // ValueType
                        .to_string()
                    on:input=on_input
                    prop:valid=valid
                />
                <div
                    class="text-red-600 text-sm mr-1.5 ml-1.5"
                    class=(["hidden"], move || !error.get())>
                    {error_text}
                </div>
            </label>
        </div>
    }
}
