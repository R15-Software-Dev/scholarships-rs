use leptos::prelude::*;
use std::str::FromStr;

#[component]
pub fn OutlinedTextField<T>(
    #[prop(optional, into)] placeholder: T,
    #[prop(optional)] value: RwSignal<T>,
    #[prop(optional)] disabled: RwSignal<bool>,
    #[prop(optional)] error: RwSignal<bool>,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] input_type: String,
) -> impl IntoView
where
    T: Default + 'static + FromStr + ToString + PartialEq + Send + Sync + Clone,
{
    // Parsing logic for non-string types which happens on each input.
    let on_input = move |e| {
        let input_value = event_target_value(&e);
        let can_parse = T::from_str(&input_value);
        if let Ok(parsed_value) = can_parse {
            value.set(parsed_value);
        } else {
            error.set(true);
            value.set(T::default());
        }
    };

    view! (
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
                    placeholder={placeholder.to_string()}
                    prop:name=name
                    prop:value={move || value.get().to_string()}  // Sets the initial value.
                    on:input=on_input
                />
            </label>
        </div>
    )
}
