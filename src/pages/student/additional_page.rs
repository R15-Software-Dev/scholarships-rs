use leptos::prelude::*;
use crate::components::DashboardButton;

#[component]
pub fn AdditionalPage() -> impl IntoView {
    view! {
        <div class="flex-1" />
        <div class="flex flex-2 flex-col gap-4 items-center m-4">
            <div class="font-bold text-3xl">"Student Additional Information Forms"</div>
            <div class="text-lg">
                "All forms listed under this tab are for additional eligibility factors that some scholarships require."
            </div>
            <div class="text-lg">
                "None of these forms are required, but they may increase the total number of scholarships that you are eligibile to be applied to."
            </div>
            <DashboardButton
                title="Scholarship Book"
                description="View scholarship requirements"
                icon="/Book.png"
                path="https://docs.google.com/document/d/1qVs0Lwvwl1bVGHznocoev3AF56KlhTfs/view"
            />
        </div>
        <div class="flex-1" />
    }
}
