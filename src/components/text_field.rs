use leptos::prelude::*;
use crate::components::ValueType;

#[component]
pub fn OutlinedTextField(
    #[prop(optional, into)] placeholder: String,
    // #[prop()] value: Subfield<TStore, TInner, ValueType>,
    #[prop()] value: RwSignal<ValueType>,
    #[prop(optional)] disabled: RwSignal<bool>,
    #[prop(optional)] error: RwSignal<bool>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] input_type: String,
    #[prop(optional, into)] error_text: RwSignal<String>
) -> impl IntoView {
    // Check if the type is one of the supported formats. This input will only accept a string
    // or a number value.
    let on_input = move |e| {
        match value.get() {
            ValueType::String(_) => {
                // Define string parse function.
                value.set(ValueType::String(event_target_value(&e)));
            },
            ValueType::Number(_) => {
                // Define number parse function.
                let temp = event_target_value(&e);
                value.set(ValueType::Number(temp.parse::<i32>().unwrap_or(0)));
            },
            _ => {
                // Create some custom function that reports failure.
                let value_type = value.get();
                println!("Input {value_type:?} is not a valid type.");
                error.set(true);
                error_text.set(format!("InputType {value_type:?} is not a valid type."));
            }
        }
    };
    
    view! {
        <div class="flex flex-1">
            <label class="flex flex-col flex-1">
                <span class="block ml-1.5 mb-0 font-bold">{label}</span>
                <input
                    class="border-2 m-1.5 p-1.5 mt-0 rounded-md bg-transparent relative flex-1
                        transition-all duration-150
                        border-red-700 bg-transparent
                        disabled:border-gray-600 disabled:pointer-events-none disabled:bg-gray-600/33"
                    r#type={input_type}
                    disabled={disabled}
                    placeholder={placeholder}
                    prop:name=name
                    prop:value={move || value.get().to_string()}  // Sets the initial value.
                    on:input=on_input
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
