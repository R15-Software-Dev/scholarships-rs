use std::str::FromStr;
use leptos::prelude::*;

#[component]
pub fn OutlinedTextField<T>(
    #[prop(default = String::from(""))] placeholder: String,
    #[prop(default = RwSignal::new(T::default()))] value: RwSignal<T>,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
    #[prop(default = String::from(""))] name: String,
    #[prop(default = String::from(""))] label: String,
    #[prop(default = String::from(""))] input_type: String,
) -> impl IntoView
where
    T: Default + 'static + FromStr + ToString + PartialEq + Send + Sync + Clone
{
    // Parsing logic for non-string types which happens on each input.
    let on_input = move |e| {
        let input_value = event_target_value(&e);
        let parsed_value = T::from_str(&input_value)
            .unwrap_or_default();
        value.set(parsed_value);
    };

    view! (
        <label>
            { label }
            <input
                class="border-2 m-1.5 p-1.5 rounded-md bg-transparent mt-6 relative transition-all duration-400"
                class=(["border-red-700", "bg-transparent"], move || !disabled.get())
                class=(["border-gray-600", "pointer-events-none", "bg-gray-600/33"], move || disabled.get())
                r#type={input_type}
                disabled={disabled}
                placeholder={placeholder}
                prop:name=name
                prop:value={move || value.get().to_string()}  // Sets the initial value.
                on:input=on_input  // Sets the value on each input.
            />
        </label>
    )
}
