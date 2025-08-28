use leptos::prelude::*;

#[component]
pub fn Row(
    children: Children
) -> impl IntoView {
    view! {
        <div class="flex flex-row gap-2 flex-1">
            {children()}
        </div>
    }
}
