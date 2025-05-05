use leptos::prelude::*;

#[component]
pub fn ActionButton(
    #[prop(optional, default = String::from("button"))]
    button_type: String,
    #[prop(optional, default = RwSignal::new(false))]
    disabled: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="m-3 p-3 rounded-lg text-white"
            class=(["bg-red-700", "hover:bg-red-800"], move || !disabled.get())
            class=("bg-gray-600", move || disabled.get())
            r#type=button_type
            disabled=move || disabled.get()
        >
        { children() }
        </button>
    }
}
