use crate::components::DashboardButton;
use leptos::prelude::*;

#[component]
pub fn StudentHomePage() -> impl IntoView {
    view! {
        <div class="flex-1" />
        <div class="flex flex-2 flex-col gap-4 items-center m-4 mt-10">
            <div class="font-bold text-3xl">"Region 15 Scholarship Application"</div>
            <div class="text-lg">
                "Welcome to the R15 general scholarship application! If you have any questions, please direct them to Sara Smith at "
                <a class="text-blue-500 underline" href="mailto:ssmith@region15.org">
                    "ssmith@region15.org"
                </a>"."
            </div>
            <div class="text-lg">"Here are some links to get you started:"</div>
            <DashboardButton
                title="Scholarship Book"
                description="View scholarship requirements"
                icon="/Book.png"
                path="https://docs.google.com/document/d/1qVs0Lwvwl1bVGHznocoev3AF56KlhTfs/view"
            />
            <DashboardButton
                title="Demographics Form"
                description="Basic student information"
                icon="/Person_Black.png"
                path="/students/demographics"
            />
            <DashboardButton
                title="Additional Forms"
                description="Extra eligibility factors"
                icon="/Form_Black.png"
                path="/students/additional"
            />
        </div>
        <div class="flex-1" />
    }
}
