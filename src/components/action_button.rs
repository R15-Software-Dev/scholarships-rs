use leptos::prelude::*;

#[component]
pub fn ActionButton(
    #[prop(default = String::from("button"), into)] button_type: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="m-1.5 p-2.5 min-w-20 rounded-lg font-semibold text-white transition-all duration-400 cursor-pointer"
            class=(["bg-red-800", "hover:bg-red-900"], move || !disabled.get())
            class=(["bg-gray-600", "pointer-events-none"], move || disabled.get())
            r#type=button_type
            disabled=move || disabled.get()
        >
        { children() }
        </button>
    }
}
