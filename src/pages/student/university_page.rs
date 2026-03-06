use leptos::prelude::*;
use crate::components::{Loading, OutlinedTextField, Select, TextFieldType, ValidatedForm};
use crate::pages::student::form_setup::use_student_form;

#[component]
pub fn StudentUniversityPage() -> impl IntoView {
    let controller = use_student_form("university");
    
    // college name, state, city, zip
    // major, study field, intended career, college_acceptance
    
    view! {
        <Show when=move || controller.data_resource.get().is_some() fallback=Loading>
            <div class="flex flex-1" />
            <div class="flex flex-col flex-2 mt-6">
                <ValidatedForm
                    title="University Information"
                    description="Here you may fill out general information about the university you plan to attend."
                    on_submit=controller.submit_action
                >
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
                    <OutlinedTextField
                        label="University State:"
                        data_map=controller.data_map
                        data_member="college_state"
                        placeholder="CT, AZ, etc."
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
                        value_list=vec!["Yes".to_string(), "No".to_string()]
                    />
                    <Select
                        label="Chosen Major:"
                        data_map=controller.data_map
                        data_member="major"
                        required=true
                        value_list=vec![
                            "Music".to_string(),
                            "Education".to_string(),
                            "Special Education".to_string(),
                            "Speech Pathology".to_string(),
                            "School Psychology".to_string(),
                            "School Counseling".to_string(),
                            "Occupational Therapy".to_string(),
                            "Physical Therapy".to_string(),
                            "Nursing".to_string(),
                            "Allied Health".to_string(),
                            "Fine/Performing Arts".to_string(),
                            "Writing/Communication".to_string(),
                            "History".to_string(),
                            "Government".to_string(),
                            "Political Science".to_string(),
                            "Social Work".to_string(),
                            "Sports Medicine".to_string(),
                            "Athletic Training".to_string(),
                            "Horticulture".to_string(),
                            "Conservation Studies".to_string(),
                            "Ecology".to_string(),
                            "Environmental Studies".to_string(),
                            "Urban Planning".to_string(),
                            "Landscaping".to_string(),
                            "Legal Studies".to_string(),
                            "Criminal Justice".to_string(),
                        ]
                    />
                    <OutlinedTextField
                        label="Intended Career:"
                        data_map=controller.data_map
                        data_member="intended_career"
                        placeholder="Engineer, Artist, etc."
                        input_type=TextFieldType::Text
                        required=false
                    />
                </ValidatedForm>
            </div>
            <div class="flex flex-1" />
        </Show>
    }
}
