use leptos::prelude::*;

#[component]
pub fn ClosedForm() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center mx-auto mt-10">
            <h1 class="text-2xl font-bold">"Form Closed"</h1>
            <p class="text-lg">
                "This form is now closed. If you believe this is an error, please contact the guidance office."
            </p>
        </div>
    }
}
