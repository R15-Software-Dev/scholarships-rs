use leptos::prelude::*;
use crate::components::{Loading, RadioList, ValidatedForm};
use crate::pages::student::form_setup::use_student_form;

#[component]
pub fn StudentSpecificRequirementsPage() -> impl IntoView {
    let controller = use_student_form("specifics");

    view! {
        <Suspense fallback=Loading>
            {move || controller.data_resource.get()
                .map(move |_| {
                    view! {
                        <div class="flex-1" />
                        <div class="flex flex-col flex-2 mt-6">
                            <ValidatedForm
                                title="Scholarship-Specific Eligibility Requirements"
                                description="These questions are specific to some scholarships. Indicate yes or no for each question."
                                on_submit=controller.submit_action
                            >
                                <RadioList
                                    label="Have you attended BAS?"
                                    data_map=controller.data_map
                                    data_member="attend_bas"
                                    items=vec!["Yes".to_string(), "No".to_string()]
                                    required=true
                                />
                                <RadioList
                                    label="Are you a member of Midd-South Catholic Church?"
                                    data_map=controller.data_map
                                    data_member="middsouth_church"
                                    items=vec!["Yes".to_string(), "No".to_string()]
                                    required=true
                                />
                                <RadioList
                                    label="Do you have a family member that is or has served in the US military?"
                                    data_map=controller.data_map
                                    data_member="family_military_service"
                                    items=vec!["Yes".to_string(), "No".to_string()]
                                    required=true
                                />
                                <RadioList
                                    label="Have you participated in Pomperaug Youth Baseball?"
                                    data_map=controller.data_map
                                    data_member="youth_baseball"
                                    items=vec!["Yes".to_string(), "No".to_string()]
                                    required=true
                                />
                                <RadioList
                                    label="Have you participated in the Panthers Aquatic Club?"
                                    data_map=controller.data_map
                                    data_member="aquatic_club"
                                    items=vec!["Yes".to_string(), "No".to_string()]
                                    required=true
                                />
                                <RadioList
                                    label="Are at least one of your parents a member of the Region 15 PEA?"
                                    data_map=controller.data_map
                                    data_member="pea_member"
                                    items=vec!["Yes".to_string(), "No".to_string()]
                                    required=true
                                />
                                <RadioList
                                    label="Have you participated in the PHS Music Program (Band, Chorus, etc)?"
                                    data_map=controller.data_map
                                    data_member="music_program"
                                    items=vec!["Yes".to_string(), "No".to_string()]
                                    required=true
                                />
                            </ValidatedForm>
                        </div>
                        <div class="flex-1" />
                    }.into_any()
                }).collect_view()
            }
        </Suspense>
    }
}