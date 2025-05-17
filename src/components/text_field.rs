use leptos::prelude::*;

#[component]
pub fn OutlinedTextField(
    #[prop(default = String::from(""))] placeholder: String,
    #[prop(default = RwSignal::new("".to_string()))] value: RwSignal<String>,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>
) -> impl IntoView {
    view! (
        <input
            class="flex-row border-2 m-1.5 p-1.5 rounded-md bg-transparent mt-6 relative transition-all duration-400"
            class=(["border-red-700"], move || !disabled.get())
            class=(["border-gray-600", "pointer-events-none"], move || disabled.get())
            r#type="text"
            disabled={disabled.get()}
            placeholder={placeholder}
            prop:value=value
        />
    )
}
