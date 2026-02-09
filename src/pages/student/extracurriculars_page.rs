use leptos::prelude::*;
use crate::components::{Loading, MultiEntry, OutlinedTextField, TextFieldType, ValidatedForm};
use crate::input;
use crate::pages::student::form_setup::use_student_form;

#[component]
pub fn StudentExtracurricularsPage() -> impl IntoView {
    let controller = use_student_form("demographics");

    view! {
        <Show when=move || controller.data_resource.get().is_some() fallback=Loading>
            <div class="flex flex-1" />
            <div class="flex flex-col flex-2 mt-6">
                <ValidatedForm
                    title="Extracurricular Information"
                    on_submit=controller.submit_action
                >
                    // Inputs here.
                    <OutlinedTextField
                        label="Total number of service hours:"
                        data_map=controller.data_map
                        data_member="service_hours"
                        placeholder="Any number..."
                        input_type=TextFieldType::Number
                        required=true
                    />
                    <MultiEntry
                        label="Extracurricular Activities:"
                        description="Leave blank if not applicable."
                        data_map=controller.data_map
                        data_member="extracurricular"
                        schema=vec![
                            input!(
                                Text, "activity_name", "Activity Name:", true, "Some activity..."
                            ),
                            input!(Number, "num_hours", "Number of hours completed:", true, "40"),
                            input!(Number, "num_weeks", "Number of weeks participated:", true, "3"),
                            input!(
                                Text, "special_involvement", "Any special involvement:", false, ""
                            ),
                            input!(
                                Checkbox, "grades", "Grades Participated:", true, ["9th", "10th", "11th", "12th"]
                            ),
                        ]
                    />
                </ValidatedForm>
            </div>
            <div class="flex flex-1" />
        </Show>
    }
}