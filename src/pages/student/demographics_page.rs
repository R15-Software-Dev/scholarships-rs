use std::collections::HashMap;
use leptos::prelude::*;
use crate::components::{OutlinedTextField, Select, TextFieldType, ValidatedForm};

#[component]
pub fn StudentDemographicsPage() -> impl IntoView {
    // This page should show a form that the students need to fill out.
    // It's a simple form, really nothing more than a contact form.
    // I'm going to test out creating this form without using a panel - we'll just use the page as 
    // it is with a specially spaced invisible div instead of a shaded container.
    
    let data_map = RwSignal::new(HashMap::new());
    
    view! {
        <div class="flex flex-1" />
        <div class="flex flex-col flex-2 mt-6">
            <ValidatedForm title="Student Demographic Form" on_submit=move || {}>
                // Inputs here.
                <OutlinedTextField
                    label="First Name:"
                    data_map=data_map
                    data_member="first_name"
                    placeholder="John"
                    input_type=TextFieldType::Text
                    required=true
                />
                <OutlinedTextField
                    label="Last Name:"
                    data_map=data_map
                    data_member="last_name"
                    placeholder="Smith"
                    input_type=TextFieldType::Text
                    required=true
                />
                // There must be a date of birth selection field here.
                <OutlinedTextField
                    label="Student ID Number:"
                    data_map=data_map
                    data_member="id_number"
                    placeholder="Your 7 digit number..."
                    input_type=TextFieldType::Number
                    required=true
                />
                <OutlinedTextField
                    label="Preferred Email:"
                    data_map=data_map
                    data_member="email"
                    placeholder="me@example.com"
                    input_type=TextFieldType::Email(vec!["*".to_string()])
                    required=true
                />
                <OutlinedTextField
                    label="Preferred Phone Number:"
                    data_map=data_map
                    data_member="phone_number"
                    placeholder="000-000-0000"
                    input_type=TextFieldType::Text
                    required=true
                />
                <OutlinedTextField
                    label="Street Address:"
                    data_map=data_map
                    data_member="street_address"
                    placeholder="234 Judd Rd"
                    input_type=TextFieldType::Text
                    required=true
                />
                <Select
                    label="Town:"
                    value_list=vec!["Southbury".to_string(), "Middlebury".to_string()]
                    data_map=data_map
                    data_member="town"
                    required=true
                />
            </ValidatedForm>
        </div>
        <div class="flex flex-1" />
    }
}
