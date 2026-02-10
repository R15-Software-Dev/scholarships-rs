use leptos::prelude::*;
use crate::components::{Loading, MultiEntry, ValidatedForm};
use crate::input;
use crate::pages::student::form_setup::use_student_form;

#[component]
pub fn StudentWorkExperiencePage() -> impl IntoView {
    let controller = use_student_form("workexp");

    view! {
        <Show when=move || controller.data_resource.get().is_some() fallback=Loading>
            <div class="flex flex-1" />
            <div class="flex flex-col flex-2 mt-6">
                <ValidatedForm title="Student Work Experience" on_submit=controller.submit_action>
                    <MultiEntry
                        label="Work Experience:"
                        description="Leave blank if not applicable."
                        data_map=controller.data_map
                        data_member="extracurricular"
                        schema=vec![
                            input!(
                                Text, "job_title", "Job Title:", true, "Waiter, Cashier, etc..."
                            ),
                            input!(Text, "employer", "Employer:", true, "Your employer's name..."),
                            input!(Number, "start_date", "Start date:", true, "40"),
                            input!(Number, "end_date", "End date:", true, "3"),
                            input!(Number, "num_hours", "Number of hours completed:", true, "120"),
                        ]
                    />
                </ValidatedForm>
            </div>
            <div class="flex flex-1" />
        </Show>
    }
}