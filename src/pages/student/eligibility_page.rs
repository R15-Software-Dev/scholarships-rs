use crate::common::{ComparisonData, ExpandableInfo, SchemaNode, SchemaType, ValueType};
use crate::components::ValueDisplay;
use crate::pages::api::students::get_all_student_data;
use crate::pages::api::{get_all_scholarship_info, get_comparison_info};
use crate::utils::get_user_claims;
use leptos::logging::debug_log;
use leptos::prelude::*;

#[component]
pub fn StudentEligibilityPage() -> impl IntoView {
    // This page will show all scholarships that the student is eligible for.
    // The scholarships are obviously pulled using a readonly API, and we'll also need to resolve
    // all of the relation IDs that are stored inside of it.
    // Then, for each scholarship found, we'll check every relation and filter out scholarships that
    // do not pass the check. We'll show some sort of simple list to indicate what scholarships passed
    // the check, and another to indicate which ones didn't.

    // My only reservation with showing the scholarships that aren't valid is that people will want
    // to know why. That's hard information to keep track of right now.

    let user_claims = get_user_claims();
    let user_id = Memo::new(move |_| user_claims.get().map(|info| info.claims.subject.clone()));

    let refresh_trigger = Trigger::new();

    let scholarships_resource = Resource::new(
        move || refresh_trigger.track(),
        async move |_| get_all_scholarship_info().await,
    );
    let relations_resource = Resource::new(
        move || refresh_trigger.track(),
        async move |_| get_comparison_info().await,
    );
    let student_resource = Resource::new(
        move || (user_id.get().unwrap_or_default(), refresh_trigger.track()),
        async move |(user_id, _)| get_all_student_data(user_id).await,
    );

    view! {
        <div class="flex-1" />
        <div>
            <Suspense fallback=move || {
                view! { <div>"Checking eligibility..."</div> }
            }>
                {move || {
                    let Some(Ok(scholarships_list)) = scholarships_resource.get() else {
                        return view! {}.into_any();
                    };
                    let Some(Ok(relations_list)) = relations_resource.get() else {
                        return view! {}.into_any();
                    };
                    let Some(Ok(student_info)) = student_resource.get() else {
                        return view! {}.into_any();
                    };
                    let valid_list = scholarships_list
                        .iter()
                        .filter_map(|scholarship| {
                            let scholarship_name = scholarship
                                .data
                                .get("name")
                                .unwrap_or(&ValueType::String(None))
                                .as_string()
                                .ok()
                                .flatten()
                                .unwrap_or_default();
                            debug_log!("Checking scholarship: {}", scholarship_name);
                            let requirements_map = scholarship
                                .data
                                .get("requirements")
                                .unwrap_or(&ValueType::Map(None))
                                .as_map()
                                .ok()
                                .flatten()
                                .unwrap_or_default()
                                .values()
                                .cloned()
                                .collect::<Vec<ValueType>>();
                            let resolved_requirements = requirements_map
                                .iter()
                                .map(|list_val| {
                                    let list = list_val
                                        .as_list()
                                        .ok()
                                        .flatten()
                                        .unwrap_or_default();
                                    list.iter()
                                        .filter_map(|v| {
                                            let id_string = v
                                                .as_string()
                                                .ok()
                                                .flatten()
                                                .unwrap_or_default();
                                            relations_list
                                                .iter()
                                                .find(|relation| relation.id == id_string)
                                        })
                                        .cloned()
                                        .collect::<Vec<ComparisonData>>()
                                })
                                .collect::<Vec<Vec<ComparisonData>>>();
                            debug_log!(
                                "Resolved requirements: {:?}", resolved_requirements.iter().flatten().map(|rel| rel.display_text.clone()).collect::<Vec<String>>()
                            );
                            let valid = resolved_requirements
                                .iter()
                                .all(|list| {
                                    list.iter()
                                        .fold(
                                            false,
                                            |prev, requirement| {
                                                let result = requirement
                                                    .compare(&student_info)
                                                    .unwrap_or(false);
                                                if prev { prev } else { result }
                                            },
                                        )
                                });
                            if valid {
                                debug_log!("Scholarship is valid.");
                                Some(scholarship.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<ExpandableInfo>>();
                    view! {
                        <div>{format!("Valid scholarships: {}", valid_list.len())}</div>
                        <For
                            each=move || valid_list.clone()
                            key=|scholarship| { scholarship.subject.clone() }
                            children=move |scholarship| {
                                let scholarship = StoredValue::new(scholarship);
                                let display_schema = SchemaNode::new(SchemaType::Map)
                                    .header("Eligible Scholarship")
                                    .child(
                                        "name",
                                        SchemaNode::new(SchemaType::String)
                                            .header("Scholarship Name"),
                                    )
                                    .child(
                                        "essay_prompt",
                                        SchemaNode::new(SchemaType::String).header("Essay Prompt"),
                                    );

                                view! {
                                    <ValueDisplay
                                        schema=display_schema
                                        data_map=scholarship.get_value().data
                                    />
                                }
                            }
                        />
                    }
                        .into_any()
                }}
            </Suspense>
        </div>
        <div class="flex-1" />
    }
}
