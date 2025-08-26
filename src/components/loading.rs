use leptos::prelude::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div>
            <img src="loading.gif" />
        </div>
    }
}
