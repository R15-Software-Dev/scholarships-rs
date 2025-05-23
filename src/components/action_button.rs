use leptos::prelude::*;

#[component]
pub fn ActionButton(
    #[prop(optional, default = String::from("button"))] button_type: String,
    #[prop(optional, default = RwSignal::new(false))] disabled: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="m-1.5 p-2 rounded-lg text-white transition-all duration-400"
            class=(["bg-red-700", "hover:bg-red-800"], move || !disabled.get())
            class=(["bg-gray-600", "pointer-events-none"], move || disabled.get())
            r#type=button_type
            disabled=move || disabled.get()
        >
        { children() }
        </button>
    }
}
