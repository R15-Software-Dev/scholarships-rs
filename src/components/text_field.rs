use leptos::prelude::*;

#[component]
pub fn OutlinedTextField(
    #[prop(default = String::from(""))] placeholder: String,
    #[prop(default = RwSignal::new("".to_string()))] value: RwSignal<String>,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
    #[prop(default = String::from(""))] name: String,
    #[prop(default = String::from(""))] label: String
) -> impl IntoView {
    view! (
        <label>
            { label }
            <input
                class="flex-row border-2 m-1.5 p-1.5 rounded-md bg-transparent mt-6 relative transition-all duration-400"
                class=(["border-red-700"], move || !disabled.get())
                class=(["border-gray-600", "pointer-events-none"], move || disabled.get())
                r#type="text"
                disabled={disabled.get()}
                placeholder={placeholder}
                prop:name=name
                bind:value=value  // This is a two-way binding - it is allowed to read *and* write.
            />
        </label>
    )
}
