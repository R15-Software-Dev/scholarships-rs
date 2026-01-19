use leptos::prelude::*;
use crate::components::Banner;

#[component]
pub fn ApplicantsPageFallback() -> impl IntoView {
    view! {
        <Banner title="R15 Scholarships" logo="/PHS_Stacked_Acronym.png" path="/providers" />
        <div class="flex flex-col gap-4 mt-3 items-center justify-center">
            <h1 class="text-3xl font-bold">"Applicants Page"</h1>

            <p>"This page is under construction! It will show all the students that are eligible for your scholarship(s)."</p>
            <p>"Come back soon!"</p>
        </div>
    }
}