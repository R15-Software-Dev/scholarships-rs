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
                "None of these forms are required, but they may increase the total number of scholarships that you are eligible to be applied to."
            </div>
            <DashboardButton
                title="Scholarship Book"
                description="View scholarship requirements"
                icon="/Book.png"
                path="https://docs.google.com/document/d/1qVs0Lwvwl1bVGHznocoev3AF56KlhTfs/view"
            />
            <DashboardButton
                title="Academic Information"
                description="Grades, tests, and more"
                path="/students/additional/academics"
            />
            <DashboardButton
                title="Athletics"
                description="Sports and outdoors"
                path="/students/additional/athletics"
            />
            <DashboardButton
                title="Work Experience"
                description="Add job information"
                path="/students/additional/work-experience"
            />
            <DashboardButton
                title="Extracurriculars"
                description="Clubs and activities"
                path="/students/additional/extracurriculars"
            />
            <DashboardButton
                title="University Information"
                description="Where are you going to school?"
                path="/students/additional/university"
            />
            <DashboardButton
                title="Family Information"
                description="General parent and sibling information"
                path="/students/additional/family-info"
            />
            <DashboardButton
                title="Financials"
                description="FAFSA SAR Upload"
                path="/students/additional/financials"
            />
        </div>
        <div class="flex-1" />
    }
}
