use leptos::prelude::*;
use crate::components::{Loading, OutlinedTextField, TextFieldType, ValidatedForm};
use crate::pages::student::form_setup::use_student_form;

#[component]
pub fn StudentAcademicsPage() -> impl IntoView {
    let controller = use_student_form("academics");

    view! {
        <Suspense fallback=Loading>
            {move || controller.data_resource.get().map(|_|
                view! {
                    <div class="flex flex-1" />
                    <div class="flex flex-col flex-2 mt-6">
                        <ValidatedForm title="Academic Information" on_submit=controller.submit_action>
                            <OutlinedTextField
                                label="Unweighted GPA:"
                                data_map=controller.data_map
                                data_member="unweighted_gpa"
                                placeholder="4.0"
                                required=true
                                input_type=TextFieldType::Number
                            />
                            <OutlinedTextField
                                label="Weighted GPA:"
                                data_map=controller.data_map
                                data_member="weighted_gpa"
                                placeholder="4.5"
                                required=true
                                input_type=TextFieldType::Number
                            />
                            <OutlinedTextField
                                label="Highest SAT Score:"
                                data_map=controller.data_map
                                data_member="sat_score"
                                placeholder="1600"
                                required=false
                                input_type=TextFieldType::Number
                            />
                            <OutlinedTextField
                                label="Highest ACT Score:"
                                data_map=controller.data_map
                                data_member="act_score"
                                placeholder="36"
                                required=false
                                input_type=TextFieldType::Number
                            />
                            <OutlinedTextField
                                label="Academic Honors:"
                                data_map=controller.data_map
                                data_member="academic_honors"
                                placeholder="TBD"
                                required=false
                                input_type=TextFieldType::Text
                            />
                        </ValidatedForm>
                    </div>
                    <div class="flex flex-1" />
                }).collect_view()
            }
        </Suspense>
    }
}