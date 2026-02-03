use leptos::prelude::*;
use std::collections::HashMap;
use crate::components::{OutlinedTextField, Select, TextFieldType, ValidatedForm};

#[component]
pub fn StudentUniversityPage() -> impl IntoView {
    let data_map = RwSignal::new(HashMap::new());
    
    // college name, state, city, zip
    // major, study field, intended career, college_acceptance
    
    view! {
        <div class="flex flex-1" />
        <div class="flex flex-col flex-2 mt-6">
            <ValidatedForm title="University Information" on_submit=move || {}>
                // Inputs here.
                <OutlinedTextField
                    label="University Name:"
                    data_map=data_map
                    data_member="college_name"
                    placeholder="University of Example"
                    input_type=TextFieldType::Number
                    required=true
                />
                <OutlinedTextField
                    label="University Street Address:"
                    data_map=data_map
                    data_member="college_city"
                    placeholder="123 Example Rd"
                    input_type=TextFieldType::Text
                    required=true
                />
                // TODO Make this a select dropdown.
                <OutlinedTextField
                    label="University State:"
                    data_map=data_map
                    data_member="college_state"
                    placeholder="The State of Panic"
                    input_type=TextFieldType::Text
                    required=true
                />
                <OutlinedTextField
                    label="University ZIP:"
                    data_map=data_map
                    data_member="college_zip"
                    placeholder="12345"
                    input_type=TextFieldType::Number
                    required=true
                />
                <Select
                    label="Have you been sent an acceptance to the university?"
                    data_map=data_map
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
                    data_map=data_map
                    data_member="major"
                    placeholder="Computer Science"
                    input_type=TextFieldType::Text
                    required=true
                />
            </ValidatedForm>
        </div>
        <div class="flex flex-1" />
    }
}
