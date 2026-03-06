use crate::components::{Loading, MultiEntry, ValidatedForm};
use crate::input;
use crate::pages::student::form_setup::use_student_form;
use leptos::prelude::*;

#[component]
pub fn StudentWorkExperiencePage() -> impl IntoView {
    let controller = use_student_form("workexp", true);

    view! {
        <Show when=move || controller.data_resource.get().is_some() fallback=Loading>
            <div class="flex flex-1" />
            <div class="flex flex-col flex-2 mt-6">
                <ValidatedForm
                    title="Student Work Experience"
                    description="Here you may add information about your past/current workplace."
                    on_submit=controller.submit_action
                    disabled=controller.submit_pending
                >
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
                            input!(
                                Text, "start_date", "Approximate start date:", true, "01/01/2000"
                            ),
                            input!(
                                Text, "end_date", "End date:", true, "01/01/2026, or current if not applicable"
                            ),
                            input!(
                                Number, "num_hours", "Approximate number of hours per week:", true, "25"
                            ),
                        ]
                    />
                </ValidatedForm>
            </div>
            <div class="flex flex-1" />
        </Show>
    }
}
