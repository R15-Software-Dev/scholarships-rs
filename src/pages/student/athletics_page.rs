use crate::components::{Loading, MultiEntry, ValidatedForm};
use crate::input;
use crate::pages::student::form_setup::use_student_form;
use leptos::prelude::*;

#[component]
pub fn StudentAthleticsPage() -> impl IntoView {
    let controller = use_student_form("athletics");

    view! {
        <Show when=move || controller.data_resource.get().is_some() fallback=Loading>
            <div class="flex flex-1" />
            <div class="flex flex-col flex-2 mt-6">
                <ValidatedForm
                    title="Athletics Information"
                    description="This information will allow you to be eligible for any scholarships that have specific sports requirements."
                    on_submit=controller.submit_action
                >
                    <MultiEntry
                        label="Athletics Activities:"
                        description="Leave blank if not applicable."
                        data_map=controller.data_map
                        data_member="sports_participation"
                        schema=vec![
                            input!(
                                Select, "sport_name", "Sport Name:", true, [
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
                            ]
                            ),
                            input!(
                                Checkbox, "grades", "Grades Participated:", true, [
                                "9th",
                                "10th",
                                "11th",
                                "12th"
                            ]
                            ),
                            input!(
                                Text, "achievements", "Special Achievements:", false, "Example 1, Example 2, etc..."
                            ),
                        ]
                    />
                </ValidatedForm>
            </div>
            <div class="flex flex-1" />
        </Show>
    }
}
