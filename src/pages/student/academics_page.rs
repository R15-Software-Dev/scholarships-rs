use crate::components::{Loading, OutlinedTextField, TextFieldType, ValidatedForm};
use crate::pages::student::form_setup::use_student_form;
use leptos::prelude::*;

#[component]
pub fn StudentAcademicsPage() -> impl IntoView {
    let controller = use_student_form("academics", true);

    view! {
        <Suspense fallback=Loading>
            {move || {
                controller
                    .data_resource
                    .get()
                    .map(|_| {
                        view! {
                            <div class="flex flex-1" />
                            <div class="flex flex-col flex-2 mt-6">
                                <ValidatedForm
                                    title="Academic Information"
                                    description="This information will allow you to be eligible for any scholarships that have specific academic/grade requirements."
                                    on_submit=controller.submit_action
                                    disabled=controller.submit_pending
                                >
                                    <OutlinedTextField
                                        label="Unweighted GPA:"
                                        disabled=controller.submit_pending
                                        data_map=controller.data_map
                                        data_member="unweighted_gpa"
                                        placeholder="4.0"
                                        required=true
                                        input_type=TextFieldType::Number
                                    />
                                    <OutlinedTextField
                                        label="Weighted GPA:"
                                        disabled=controller.submit_pending
                                        data_map=controller.data_map
                                        data_member="weighted_gpa"
                                        placeholder="4.5"
                                        required=true
                                        input_type=TextFieldType::Number
                                    />
                                    <OutlinedTextField
                                        label="Highest SAT Score:"
                                        disabled=controller.submit_pending
                                        data_map=controller.data_map
                                        data_member="sat_score"
                                        placeholder="1600"
                                        required=false
                                        input_type=TextFieldType::Number
                                    />
                                    <OutlinedTextField
                                        label="Highest ACT Score:"
                                        disabled=controller.submit_pending
                                        data_map=controller.data_map
                                        data_member="act_score"
                                        placeholder="36"
                                        required=false
                                        input_type=TextFieldType::Number
                                    />
                                    <OutlinedTextField
                                        label="Academic Honors:"
                                        disabled=controller.submit_pending
                                        data_map=controller.data_map
                                        data_member="academic_honors"
                                        placeholder="High Honors, etc."
                                        required=false
                                        input_type=TextFieldType::Text
                                    />
                                </ValidatedForm>
                            </div>
                            <div class="flex flex-1" />
                        }
                    })
                    .collect_view()
            }}
        </Suspense>
    }
}
