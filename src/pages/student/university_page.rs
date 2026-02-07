use leptos::prelude::*;
use crate::components::{Loading, OutlinedTextField, Select, TextFieldType, ValidatedForm};
use crate::pages::student::form_setup::use_student_form;

#[component]
pub fn StudentUniversityPage() -> impl IntoView {
    let controller = use_student_form("university");
    
    // college name, state, city, zip
    // major, study field, intended career, college_acceptance
    
    view! {
        <Show
            when=move || controller.data_resource.get().is_some()
            fallback=Loading
        >
            <div class="flex flex-1" />
            <div class="flex flex-col flex-2 mt-6">
                <ValidatedForm title="University Information" on_submit=controller.submit_action>
                    // Inputs here.
                    <OutlinedTextField
                        label="University Name:"
                        data_map=controller.data_map
                        data_member="college_name"
                        placeholder="University of Example"
                        input_type=TextFieldType::Text
                        required=true
                    />
                    <OutlinedTextField
                        label="University Street Address:"
                        data_map=controller.data_map
                        data_member="college_city"
                        placeholder="123 Example Rd"
                        input_type=TextFieldType::Text
                        required=true
                    />
                    // TODO Make this a select dropdown.
                    <OutlinedTextField
                        label="University State:"
                        data_map=controller.data_map
                        data_member="college_state"
                        placeholder="The State of Panic"
                        input_type=TextFieldType::Text
                        required=true
                    />
                    <OutlinedTextField
                        label="University ZIP:"
                        data_map=controller.data_map
                        data_member="college_zip"
                        placeholder="12345"
                        input_type=TextFieldType::Text
                        required=true
                    />
                    <Select
                        label="Have you been sent an acceptance to the university?"
                        data_map=controller.data_map
                        data_member="college_acceptance"
                        required=true
                        value_list=vec![
                            "Yes".to_string(),
                            "No".to_string()
                        ]
                    />
                    // TODO Make this a select dropdown.
                    <OutlinedTextField
                        label="Chosen Major:"
                        data_map=controller.data_map
                        data_member="major"
                        placeholder="Computer Science"
                        input_type=TextFieldType::Text
                        required=true
                    />
                </ValidatedForm>
            </div>
            <div class="flex flex-1" />
        </Show>
    }
}
