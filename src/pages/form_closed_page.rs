use crate::components::Banner;
use leptos::prelude::*;

#[component]
pub fn FormClosedPage() -> impl IntoView {
    // This page simply shows a message saying a form is closed. It does not make any distinction
    // about which form is closed right now.

    view! {
        <Banner title="R15 Scholarships" logo="/PHS_Stacked_Acronym.png" path="/providers" />
        <div class="flex flex-1 flex-col gap-4 items-center m-4">
            <div class="font-bold text-3xl">"This form has been closed!"</div>
            <div class="text-lg">
                "If you believe this is an error, please contact Sara Smith at "
                <a href="mailto:ssmith@region15.org">"ssmith@region15.org"</a>
            </div>
        </div>
    }
}
