use leptos::prelude::*;
use crate::components::{Loading, OutlinedTextField, TextFieldType, ValidatedForm};
use crate::pages::student::form_setup::use_student_form;

#[component]
pub fn StudentFamilyPage() -> impl IntoView {
    let controller = use_student_form("family");
    
    view! {
        <Show when=move || controller.data_resource.get().is_some() fallback=Loading>
            <div class="flex flex-1" />
            <div class="flex flex-col flex-2 mt-6">
                <ValidatedForm title="Family Information" on_submit=controller.submit_action>
                    // Inputs here.
                    <OutlinedTextField
                        label="Total number of children in family:"
                        data_map=controller.data_map
                        data_member="num_children"
                        placeholder="Any number..."
                        input_type=TextFieldType::Number
                        required=true
                    />
                    <OutlinedTextField
                        label="Total number of children currently attending college:"
                        data_map=controller.data_map
                        data_member="num_children_college"
                        placeholder="Any number..."
                        input_type=TextFieldType::Number
                        required=true
                    />
                    <OutlinedTextField
                        label="Parent/Guardian 1 Name:"
                        data_map=controller.data_map
                        data_member="parent_one_name"
                        placeholder="John Smith"
                        input_type=TextFieldType::Text
                        required=true
                    />
                    // TODO Could this be a select dropdown?
                    <OutlinedTextField
                        label="Parent/Guardian 1 Relationship:"
                        data_map=controller.data_map
                        data_member="parent_one_relationship"
                        placeholder="Mother/Father"
                        input_type=TextFieldType::Text
                        required=true
                    />
                    // TODO Get a better example
                    <OutlinedTextField
                        label="Parent/Guardian 1 Occupation:"
                        data_map=controller.data_map
                        data_member="parent_one_occupation"
                        placeholder="Milkman"
                        input_type=TextFieldType::Text
                        required=true
                    />
                    <OutlinedTextField
                        label="Parent/Guardian 1 Employer:"
                        data_map=controller.data_map
                        data_member="parent_one_employer"
                        placeholder="Example Employer"
                        input_type=TextFieldType::Text
                        required=true
                    />

                    <OutlinedTextField
                        label="Parent/Guardian 2 Name:"
                        data_map=controller.data_map
                        data_member="parent_two_name"
                        placeholder="John Smith"
                        input_type=TextFieldType::Text
                        required=false
                    />
                    // TODO Could this be a select dropdown?
                    <OutlinedTextField
                        label="Parent/Guardian 2 Relationship:"
                        data_map=controller.data_map
                        data_member="parent_two_relationship"
                        placeholder="Mother/Father"
                        input_type=TextFieldType::Text
                        required=false
                    />
                    // TODO Get a better example
                    <OutlinedTextField
                        label="Parent/Guardian 2 Occupation:"
                        data_map=controller.data_map
                        data_member="parent_two_occupation"
                        placeholder="Milkman"
                        input_type=TextFieldType::Text
                        required=false
                    />
                    <OutlinedTextField
                        label="Parent/Guardian 2 Employer:"
                        data_map=controller.data_map
                        data_member="parent_two_employer"
                        placeholder="Example Employer"
                        input_type=TextFieldType::Text
                        required=false
                    />
                </ValidatedForm>
            </div>
            <div class="flex flex-1" />
        </Show>
    }
}