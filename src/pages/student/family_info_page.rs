use std::collections::HashMap;
use leptos::prelude::*;
use crate::components::{OutlinedTextField, TextFieldType, ValidatedForm};

#[component]
pub fn StudentFamilyPage() -> impl IntoView {
    let data_map = RwSignal::new(HashMap::new());
    
    view! {
        <div class="flex flex-1" />
        <div class="flex flex-col flex-2 mt-6">
            <ValidatedForm title="Family Information" on_submit=move || {}>
                // Inputs here.
                <OutlinedTextField
                    label="Total number of children in family:"
                    data_map=data_map
                    data_member="num_children"
                    placeholder="Any number..."
                    input_type=TextFieldType::Number
                    required=true
                />
                <OutlinedTextField
                    label="Total number of children currently attending college:"
                    data_map=data_map
                    data_member="num_children_college"
                    placeholder="Any number..."
                    input_type=TextFieldType::Number
                    required=true
                />
                <OutlinedTextField
                    label="Parent/Guardian 1 Name:"
                    data_map=data_map
                    data_member="parent_one_name"
                    placeholder="John Smith"
                    input_type=TextFieldType::Text
                    required=true
                />
                // TODO Could this be a select dropdown?
                <OutlinedTextField
                    label="Parent/Guardian 1 Relationship:"
                    data_map=data_map
                    data_member="parent_one_relationship"
                    placeholder="Mother/Father"
                    input_type=TextFieldType::Text
                    required=true
                />
                // TODO Get a better example
                <OutlinedTextField
                    label="Parent/Guardian 1 Occupation:"
                    data_map=data_map
                    data_member="parent_one_occupation"
                    placeholder="Milkman"
                    input_type=TextFieldType::Text
                    required=true
                />
                <OutlinedTextField
                    label="Parent/Guardian 1 Employer:"
                    data_map=data_map
                    data_member="parent_one_employer"
                    placeholder="Example Employer"
                    input_type=TextFieldType::Text
                    required=true
                />
        
                <OutlinedTextField
                    label="Parent/Guardian 2 Name:"
                    data_map=data_map
                    data_member="parent_one_name"
                    placeholder="John Smith"
                    input_type=TextFieldType::Text
                    required=false
                />
                // TODO Could this be a select dropdown?
                <OutlinedTextField
                    label="Parent/Guardian 2 Relationship:"
                    data_map=data_map
                    data_member="parent_one_relationship"
                    placeholder="Mother/Father"
                    input_type=TextFieldType::Text
                    required=false
                />
                // TODO Get a better example
                <OutlinedTextField
                    label="Parent/Guardian 2 Occupation:"
                    data_map=data_map
                    data_member="parent_one_occupation"
                    placeholder="Milkman"
                    input_type=TextFieldType::Text
                    required=false
                />
                <OutlinedTextField
                    label="Parent/Guardian 2 Employer:"
                    data_map=data_map
                    data_member="parent_one_employer"
                    placeholder="Example Employer"
                    input_type=TextFieldType::Text
                    required=false
                />
            </ValidatedForm>
        </div>
        <div class="flex flex-1" />
    }
}