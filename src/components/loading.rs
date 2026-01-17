use leptos::prelude::*;

#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="size-20 mx-auto">
            <img src="/loading.gif" />
        </div>
    }
}
