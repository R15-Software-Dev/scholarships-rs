use std::collections::HashMap;
use leptos::prelude::*;
use crate::components::{MultiEntry, ValidatedForm};
use crate::input;

#[component]
pub fn StudentAthleticsPage() -> impl IntoView {
    let data_map = RwSignal::new(HashMap::new());

    view! {
        <div class="flex flex-1" />
        <div class="flex flex-col flex-2 mt-6">
            <ValidatedForm title="Athletics Information" on_submit=move || {}>
                <MultiEntry
                    data_map=data_map
                    data_member="athletic_participation"
                    schema=vec![
                        input!(Select, "sport_name", "Sport Name:", true, [
                            "Football",
                            "Soccer",
                            "Cheerleading",
                            "Field Hockey",
                            "Swimming",
                            "Golf",
                            "Basketball",
                            "Track",
                            "Gymnastics",
                            "Ice Hockey",
                            "Ski",
                            "Wrestling",
                            "Lacrosse",
                            "Softball",
                            "Tennis",
                            "Baseball"
                        ]),
                        input!(Checkbox, "grades", "Grades Participated:", true, [
                            "9th",
                            "10th",
                            "11th",
                            "12th"
                        ]),
                        input!(Text, "achievements", "Special Achievements:", false, "Example 1, Example 2, etc...")
                    ]
                />
            </ValidatedForm>
        </div>
        <div class="flex flex-1" />
    }
}