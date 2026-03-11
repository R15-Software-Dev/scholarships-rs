use crate::common::{ComparisonData, ExpandableInfo, ValueType};
use crate::components::{FileDrop, Header, Loading};
use crate::pages::api::files::list_files;
use crate::pages::api::students::get_all_student_data;
use crate::pages::api::{get_all_scholarship_info, get_comparison_info};
use crate::utils::get_user_claims;
use leptos::logging::debug_log;
use leptos::prelude::*;
use leptos_oidc::AuthSignal;

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

    let auth = expect_context::<AuthSignal>();
    let user_claims = get_user_claims();
    let user_id = Memo::new(move |_| user_claims.get().map(|info| info.claims.subject.clone()));

    let form_id = StoredValue::new("scholarship_essays".to_string());

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
        <div class="flex-2 mb-4 flex flex-col">
            <Suspense fallback=Loading>
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
                                                    .unwrap_or_else(|err| {
                                                        debug_log!("Requirement failed: {err}");
                                                        false
                                                    });
                                                if prev { prev } else { result }
                                            },
                                        )
                                });
                            if valid {
                                debug_log!("Scholarship is valid.");
                                let scholarship_id = StoredValue::new(scholarship.subject.clone());
                                let resource = Resource::new(
                                    move || (
                                        auth
                                            .try_with(|a| a.authenticated().map(|a| a.access_token()))
                                            .flatten(),
                                        form_id.get_value(),
                                        scholarship_id.get_value(),
                                    ),
                                    async move |(access_token, form_id, scholarship_id)| {
                                        list_files(
                                                access_token.unwrap_or_default(),
                                                form_id,
                                                scholarship_id,
                                            )
                                            .await
                                    },
                                );
                                Some((scholarship.clone(), resource))
                            } else {
                                None
                            }
                        })
                        .collect::<
                            Vec<(ExpandableInfo, Resource<Result<Vec<String>, ServerFnError>>)>,
                        >();

                    view! {
                        <Header
                            title="Eligible Scholarships"
                            description="This is the list of scholarships that you are eligible for, as well as those that require essays.
                            Once you have fulfilled the essay requirement, you will be automatically applied for that scholarship.
                            If there is no essay requirement, you have already been applied."
                        />
                        <div>
                            <For
                                each=move || valid_list.clone()
                                key=|(scholarship, _)| { scholarship.subject.clone() }
                                children=move |(scholarship, resource)| {
                                    let scholarship_id = StoredValue::new(
                                        scholarship.subject.clone(),
                                    );
                                    let scholarship_name = StoredValue::new(
                                        scholarship
                                            .data
                                            .get("name")
                                            .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                                            .unwrap_or_default(),
                                    );
                                    let scholarship_essay = StoredValue::new(
                                        scholarship
                                            .data
                                            .get("essay_prompt")
                                            .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                                            .unwrap_or_default(),
                                    );

                                    view! {
                                        <div class="m-1.5 flex flex-col gap-2 shadow-lg rounded-lg">
                                            <div class="rounded-t-lg bg-red-900 p-2 text-white font-bold">
                                                {scholarship_name.get_value()}
                                            </div>

                                            <div class="p-2 flex flex-col gap-2">
                                                <Show
                                                    when=move || !scholarship_essay.get_value().is_empty()
                                                    fallback=move || {
                                                        view! {
                                                            <div>"This scholarship does not require an essay."</div>
                                                        }
                                                    }
                                                >
                                                    <div>{scholarship_essay.get_value()}</div>
                                                    <FileDrop
                                                        form_id=form_id.get_value()
                                                        name=scholarship_id.get_value()
                                                        existing_files=resource
                                                    />
                                                </Show>
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }
                        .into_any()
                }}
            </Suspense>
        </div>
        <div class="flex-1" />
    }
}
