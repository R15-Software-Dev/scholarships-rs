use leptos::prelude::*;

#[component]
pub fn Checkbox(
    #[prop()] checked: ReadSignal<bool>,
    #[prop()] on_change: Callback<bool>,
    #[prop()] label: String,
    #[prop()] id: String
) -> impl IntoView {
    let label_id = id.clone();
    view! {
        <label for=label_id>
            <input
                type="checkbox"
                id={id}
            />
        </label>
    }
}
