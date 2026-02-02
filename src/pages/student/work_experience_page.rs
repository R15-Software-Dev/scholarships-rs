use std::collections::HashMap;
use leptos::prelude::*;
use crate::components::{MultiEntry, ValidatedForm};
use crate::input;

#[component]
pub fn StudentWorkExperiencePage() -> impl IntoView {
    let data_map = RwSignal::new(HashMap::new());

    view! {
        <div class="flex flex-1" />
        <div class="flex flex-col flex-2 mt-6">
            <ValidatedForm title="Student Work Experience" on_submit=move || {}>
                <MultiEntry
                    label="Work Experience:"
                    description="Leave blank if not applicable."
                    data_map=data_map
                    data_member="extracurricular"
                    schema=vec![
                        input!(Text, "job_title", "Job Title:", true, "Waiter, Cashier, etc..."),
                        input!(Text, "employer", "Employer:", true, "Your employer's name..."),
                        input!(Number, "start_date", "Start date:", true, "40"),
                        input!(Number, "end_date", "End date:", true, "3"),
                        input!(Number, "num_hours", "Number of hours completed:", true, "120"),
                    ]
                />
            </ValidatedForm>
        </div>
        <div class="flex flex-1" />
    }
}