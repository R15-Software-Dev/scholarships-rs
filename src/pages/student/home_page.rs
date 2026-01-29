use leptos::prelude::*;

#[component]
pub fn StudentHomePage() -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col gap-4 items-center m-4">
            <div class="font-bold text-3xl">"Student Home Page"</div>
            <div class="text-lg">
                "This is just a placeholder for now. We'll show some redirect buttons."
            </div>
        </div>
    }
}
