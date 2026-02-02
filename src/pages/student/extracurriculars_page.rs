use std::collections::HashMap;
use leptos::prelude::*;
use crate::components::{MultiEntry, OutlinedTextField, TextFieldType, ValidatedForm};
use crate::input;

#[component]
pub fn StudentExtracurricularsPage() -> impl IntoView {
    let data_map = RwSignal::new(HashMap::new());

    view! {
        <div class="flex flex-1" />
        <div class="flex flex-col flex-2 mt-6">
            <ValidatedForm title="Extracurricular Information" on_submit=move || {}>
                // Inputs here.
                <OutlinedTextField
                    label="Total number of service hours:"
                    data_map=data_map
                    data_member="service_hours"
                    placeholder="Any number..."
                    input_type=TextFieldType::Number
                    required=true
                />
                <MultiEntry
                    label="Extracurricular Activities:"
                    description="Leave blank if not applicable."
                    data_map=data_map
                    data_member="extracurricular"
                    schema=vec![
                        input!(Text, "activity_name", "Activity Name:", true, "Some activity..."),
                        input!(Number, "num_hours", "Number of hours completed:", true, "40"),
                        input!(Number, "num_weeks", "Number of weeks participated:", true, "3"),
                        input!(Text, "special_involvement", "Any special involvement:", false, ""),
                        input!(Checkbox, "grades", "Grades Participated:", true, ["9th", "10th", "11th", "12th"])
                    ]
                />
            </ValidatedForm>
        </div>
        <div class="flex flex-1" />
    }
}