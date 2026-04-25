use crate::common::{
    ComparisonData, ExpandableInfo, SchemaContainerStyle, SchemaHeaderStyle, SchemaNode,
    SchemaType, ValueType,
};
use crate::components::{ActionButton, Banner, DataDisplay, Loading};
use crate::pages::{StudentInformationDialog, UnauthenticatedPage};
use crate::pages::api::get_comparison_info;
use crate::pages::api::students::{GetStudentFiles, admin_get_all_input_files, get_student_data};
use base64::Engine;
use leptos::ev::{Event, MouseEvent};
use leptos::html::Dialog;
use leptos::logging::debug_log;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys;
use leptos::web_sys::{HtmlAnchorElement, ScrollBehavior};
use leptos_icons::Icon;
use leptos_oidc::{AuthLoaded, AuthSignal, Authenticated};
use leptos_router::components::Outlet;
use leptos_router::hooks::use_navigate;
use std::collections::HashMap;
use leptos_router::NavigateOptions;

#[component]
pub fn AdminApplicantsPageFallback() -> impl IntoView {
    view! {
        <div class="flex flex-col flex-1 gap-2">
            <h1 class="flex-1 text-center font-bold text-2xl">"Scholarship Applicants"</h1>
            <p class="text-center text-lg">
                "Please select a scholarship from the left to view the eligible applicants."
            </p>
        </div>
    }
}

#[component]
pub fn AdminApplicantsPageShell() -> impl IntoView {
    // This shell needs to simply contain everything that will stay consistent across paths.
    // The path will change to contain each scholarship's id, which will directly affect the content
    // shown in a subroute for this page.

    // As a result, this page likely only needs to contain the banner and the list of scholarships.
    // The content will be shown within the right side, but only as a result of some subroutes,
    // each of which is still considered a single page.

    view! {
        <div class="flex flex-row mt-5">
            <div class="flex-1" />
            <div class="flex-3 flex flex-row items-start">
                <AdminApplicantsScholarshipList />
                <div class="flex-3 text-center">
                    <Outlet />
                </div>
            </div>
            <div class="flex-1" />
        </div>
    }
}

#[component]
fn AdminApplicantsScholarshipList() -> impl IntoView {
    use crate::common::ExpandableInfo;

    #[server]
    async fn admin_get_provider_scholarships(
        access_token: String,
    ) -> Result<Vec<ExpandableInfo>, ServerFnError> {
        use crate::pages::api::SCHOLARSHIPS_TABLE;
        use crate::pages::api::tokens::validate_and_get_token_info;

        let claims = validate_and_get_token_info(access_token, "us-east-1_rvCU4Xy4j", "us-east-1").await?;

        // Get the information from the database.
        let client = crate::utils::server::create_dynamo_client().await;

        let output = client
            .scan()
            .table_name(SCHOLARSHIPS_TABLE)
            .send()
            .await?
            .items
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| serde_dynamo::from_item(item).ok())
            .collect::<Vec<ExpandableInfo>>();

        Ok(output)
    }

    // Use access token for user identification
    let auth = expect_context::<AuthSignal>();
    let access_token =
        Memo::new(move |_| auth.with(|auth| auth.authenticated().map(|a| a.access_token())));

    // We need to get the scholarships from the API. We don't need the whole scholarship, just the
    // name and ID.
    let trigger = Trigger::new();
    let scholarships_res = Resource::new(
        move || (trigger.track(), access_token.get()),
        async move |(_, access_token)| {
            admin_get_provider_scholarships(access_token.unwrap_or_default()).await
        },
    );

    let navigate = use_navigate();

    // This closure returns another closure that's using the correct scholarship ID.
    // It avoids having to define it within the Transition component. It's wrapped in a Callback
    // because this makes it Send + Sync.
    let on_view_click = Callback::new(move |scholarship_id: String| {
        let path = format!("/admin/applicants/{}", scholarship_id);
        navigate(&*path, Default::default());
    });

    view! {
        <div class="flex flex-col flex-1 rounded-md shadow-lg/33 p-2 gap-3">
            <h2 class="text-xl font-bold flex-1 text-center">"Scholarships"</h2>
            <Transition fallback=Loading>
                {move || {
                    scholarships_res
                        .get()
                        .map(|items_res| {
                            let items = match items_res {
                                Ok(items) => items,
                                Err(e) => {
                                    return view! {
                                        <div>
                                            {format!("Couldn't get scholarships: {}", e.to_string())}
                                        </div>
                                    }
                                        .into_any();
                                }
                            };

                            view! {
                                <For
                                    each=move || items.clone()
                                    key=|item| item.subject.clone()
                                    let(item)
                                >
                                    <ApplicantsScholarshipEntry
                                        item=item
                                        on_view_click=on_view_click
                                    />
                                </For>
                            }
                                .into_any()
                        })
                        .collect_view()
                }}
            </Transition>
        </div>
    }
}

#[component]
fn ApplicantsScholarshipEntry(
    #[prop()] item: ExpandableInfo,
    #[prop(into)] on_view_click: Callback<String, ()>,
) -> impl IntoView {
    // Store the item's subject.
    let subject = StoredValue::new(item.subject);
    let scholarship_name = Memo::new(move |_| {
        item.data
            .get("name")
            .map(|name| name.as_string().ok().flatten())
            .flatten()
            .unwrap_or("<unnamed scholarship>".to_string())
    });

    let on_click = move |_| {
        on_view_click.run(subject.get_value());
    };

    view! {
        <div class="flex flex-col rounded-md shadow-md hover:shadow-lg/33 transition-shadow">
            <div class="text-center p-2">{scholarship_name}</div>
            <div
                class="flex-1 font-bold text-white rounded-b-md text-center cursor-pointer bg-red-800 hover:bg-red-900 p-2 transition-bg"
                on:click=on_click
            >
                "View Applicants"
            </div>
        </div>
    }
}

#[component]
pub fn AdminApplicantsStudentList() -> impl IntoView {
    use crate::pages::api::get_scholarship_info;
    use crate::pages::api::students::admin_get_completed_students;
    use leptos_router::hooks::use_params;
    use leptos_router::params::Params;

    /// The parameters for the page that displays this component. This may be moved to the page-level
    /// rather than the component-level.
    #[derive(Params, PartialEq)]
    struct StudentListParams {
        scholarship_id: Option<String>,
    }

    // This is in charge of finding all student information and running the comparisons.
    // It will then display all the eligible students.

    let params = use_params::<StudentListParams>();
    let auth = expect_context::<AuthSignal>();
    let access_token =
        Memo::new(move |_| auth.with(|a| a.authenticated().map(|a| a.access_token())));

    let scholarship_id = Memo::new(move |_| {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.scholarship_id.clone())
    });

    #[server]
    async fn get_eligibility_info(
        token: String,
        scholarship_id: String,
    ) -> Result<
        (
            HashMap<String, HashMap<String, ValueType>>,
            ExpandableInfo,
            Vec<ComparisonData>,
        ),
        ServerFnError,
    > {
        let (students, scholarships, requirements_list) = tokio::join!(
            admin_get_completed_students(token),
            get_scholarship_info(scholarship_id),
            get_comparison_info()
        );

        Ok((students?, scholarships?, requirements_list?))
    }

    let resource = Resource::new(
        move || (access_token.get(), scholarship_id.get()),
        move |(access_token, scholarship_id)| async move {
            let Some(token) = access_token else {
                return Err(ServerFnError::new("Couldn't find access token"));
            };
            let Some(scholarship_id) = scholarship_id else {
                return Err(ServerFnError::new("Couldn't find scholarship ID"));
            };

            get_eligibility_info(token, scholarship_id).await
        },
    );

    view! {
        <div class="flex flex-col flex-1">
            <Suspense fallback=Loading>
                {move || {
                    resource
                        .get()
                        .map(|result| {
                            match result {
                                Ok((students, scholarship, requirements_list)) => {
                                    view! {
                                        <h1 class="text-2xl font-bold">"Eligible Students"</h1>
                                        <div class="text-lg">
                                            "Click on a student's name to view their information and download any requested files."
                                        </div>
                                        <div class="text-lg">
                                            "If your scholarship requires a transcript, please send an email to Sara Smith at "
                                            <a
                                                class="text-blue-500 underline"
                                                href="mailto:ssmith@region15.org"
                                            >
                                                "ssmith@region15.org"
                                            </a>"."
                                        </div>
                                        <AdminApplicantsStudentListView
                                            access_token=access_token
                                            students=students
                                            scholarship=scholarship
                                            requirements=requirements_list
                                        />
                                    }
                                        .into_any()
                                }
                                Err(e) => {
                                    view! {
                                        <div>
                                            "Error while getting eligible students: "{e.to_string()}
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn AdminApplicantsStudentListView(
    #[prop(into)] access_token: Signal<Option<String>>,
    #[prop(into)] students: Signal<HashMap<String, HashMap<String, ValueType>>>,
    #[prop(into)] scholarship: Signal<ExpandableInfo>,
    #[prop(into)] requirements: Signal<Vec<ComparisonData>>,
) -> impl IntoView {
    let eligible_students = RwSignal::new(HashMap::new());
    let error_msg = RwSignal::new(String::new());
    // let (current_student_id, set_current_student_id) = query_signal::<String>("student_id");
    let current_student_id = RwSignal::new(None);
    let navigate_student = Callback::new(move |id: String| {
        let options = web_sys::ScrollToOptions::new();
        options.set_top(0.0);
        options.set_behavior(ScrollBehavior::Smooth);

        if let Some(window) = web_sys::window() {
            window.scroll_to_with_scroll_to_options(&options);
        }

        current_student_id.set(Some(id));
    });
    let dialog_visible =
        Signal::derive(move || !current_student_id.get().unwrap_or_default().is_empty());
    let on_dialog_close = Callback::new(move |_| {
        current_student_id.set(None);
    });

    let fafsa_required = Memo::new(move |_| {
        let required = scholarship
            .get()
            .data
            .get("fafsa_required")
            .map(|v| v.as_string().ok().flatten())
            .flatten()
            .unwrap_or_default();

        required == "Yes"
    });

    let essay_required = Memo::new(move |_| {
        let prompt = scholarship
            .get()
            .data
            .get("essay_prompt")
            .map(|v| v.as_string().ok().flatten())
            .flatten()
            .unwrap_or_default();

        !prompt.is_empty()
    });

    let fafsa_resource = Resource::new(
        move || (fafsa_required.get(), access_token.get()),
        move |(fafsa_required, access_token)| async move {
            // Only get this information if the FAFSA is required. Otherwise, return a blank result.
            let Some(access_token) = access_token else {
                return Err(ServerFnError::new("Couldn't find access token"));
            };
            if !fafsa_required {
                return Ok(HashMap::new());
            }

            admin_get_all_input_files(
                access_token,
                "financial_info".to_string(),
                "fafsa".to_string(),
            )
                .await
        },
    );
    let essay_resource = Resource::new(
        move || (essay_required.get(), access_token.get()),
        move |(essay_required, access_token)| async move {
            // Only get this information if the scholarship requested it. Otherwise, return a blank
            // result.
            let Some(access_token) = access_token else {
                return Err(ServerFnError::new("Couldn't find access token"));
            };
            if !essay_required {
                return Ok(HashMap::new());
            }

            admin_get_all_input_files(
                access_token,
                "scholarship_essays".to_string(),
                scholarship.get().subject,
            )
                .await
        },
    );

    fn get_eligible_students(
        students: HashMap<String, HashMap<String, ValueType>>,
        scholarship: ExpandableInfo,
        requirements: Vec<ComparisonData>,
        essay_required: bool,
        fafsa_required: bool,
        fafsa_list: HashMap<String, Vec<String>>,
        essay_list: HashMap<String, Vec<String>>,
    ) -> Result<HashMap<String, HashMap<String, ValueType>>, Error> {
        // Get the full map of requirements. Each key contains a list of IDs.
        let scholarship_requirements = scholarship
            .data
            .get("requirements")
            .map(|val| val.as_map().ok()?)
            .flatten()
            .unwrap_or_default();

        // Get the lists of requirements. Maps into string IDs and discards invalid ValueTypes.
        let requirement_lists = scholarship_requirements
            .into_iter()
            .filter_map(|(_, list_val)| {
                let list = list_val.as_list().ok().flatten()?;
                let string_list = list
                    .into_iter()
                    .filter_map(|v| v.as_string().ok().flatten())
                    .collect::<Vec<String>>();

                Some(string_list)
            })
            .collect::<Vec<Vec<String>>>();

        // Resolve the requirement IDs. Discards all IDs that do not resolve.
        let resolved_requirements = requirement_lists
            .into_iter()
            .map(|list| {
                list.into_iter()
                    .filter_map(|id| {
                        requirements
                            .iter()
                            .cloned()
                            .find(|requirement| requirement.id == id)
                    })
                    .collect::<Vec<ComparisonData>>()
            })
            .collect::<Vec<Vec<ComparisonData>>>();

        // Check student eligibility. These are all students that have completed the demographics
        // forms.
        let eligible_students = students
            .into_iter()
            .filter_map(|(id, student)| {
                // The student needs to pass one requirement from each list.
                // The easiest ones to check first are whether the scholarship requires a student
                // essay and/or their fafsa information. If the student does not have these things,
                // they are skipped.

                if fafsa_required && !fafsa_list.contains_key(&id) {
                    // Get the student files related to their fafsa and continue. Return false if not there.
                    debug_log!(
                        "Student with id {id} failed - FAFSA is required but was not found."
                    );
                    return None;
                }

                if essay_required && !essay_list.contains_key(&id) {
                    // Get the student's files related to their essays and continue. Return false if not there.
                    debug_log!(
                        "Student with id {id} failed - essay is required but was not found."
                    );
                    return None;
                }

                let result = resolved_requirements.iter().all(|list| {
                    if list.is_empty() {
                        return true;
                    }
                    list.iter().any(|requirement| {
                        let result = requirement.compare(&student).unwrap_or(false);

                        if !result {
                            debug_log!(
                                "Student with id {id} failed - requirement with id {} failed.",
                                requirement.id
                            );
                        }

                        result
                    })
                });

                if result { Some((id, student)) } else { None }
            })
            .collect::<HashMap<String, HashMap<String, ValueType>>>();

        // debug_log!("Total number of eligible students: {}", eligible_students.len());

        Ok(eligible_students)
    }

    Effect::new(move || {
        // We have to wait until we've gotten fafsa/essay requirements.
        debug_log!("Waiting on essay and FAFSA resources...");
        if let (Some(fafsa_list), Some(essay_list)) = (fafsa_resource.get(), essay_resource.get()) {
            debug_log!(
                "Running eligibility check on {} students",
                students.get().len()
            );
            let eligible_res = get_eligible_students(
                students.get(),
                scholarship.get(),
                requirements.get(),
                essay_required.get(),
                fafsa_required.get(),
                fafsa_list.unwrap_or_default(),
                essay_list.unwrap_or_default(),
            );

            match eligible_res {
                Ok(list) => {
                    debug_log!("Total number of eligible students: {}", list.len());
                    eligible_students.set(list);
                }
                Err(e) => error_msg.set(format!("Failed to get eligible students list: {e}")),
            }
        }
    });

    // This is actually just a display of all the information from the students list.
    // As defined in the ApplicantsStudentList component, this will already be wrapped in a
    // flex container.
    view! {
        <StudentInformationDialog
            scholarship_id=scholarship.get().subject
            student_id=current_student_id
            visible=dialog_visible
            on_close=on_dialog_close
            essay_required=essay_required
            fafsa_required=fafsa_required
        />
        <Show when=move || !error_msg.get().is_empty()>
            <div>"Error checking student eligibility: "{error_msg}</div>
        </Show>
        <div class="m-3 flex flex-col">
            <div class="flex flex-row p-1 border-b-1 border-black">
                <div class="pl-5 flex-1 text-lg font-bold text-left">"Last Name"</div>
                <div class="pr-5 flex-1 text-lg font-bold text-left">"First Name"</div>
            </div>
            <For
                each=move || eligible_students.get()
                key=|(id, _)| id.clone()
                children=move |(student_id, student)| {
                    let student = StoredValue::new(student);
                    let first_name = Memo::new(move |_| {
                        student
                            .get_value()
                            .get("first_name")
                            .map(|n| n.as_string().ok().flatten())
                            .unwrap_or_default()
                    });
                    let last_name = Memo::new(move |_| {
                        student
                            .get_value()
                            .get("last_name")
                            .map(|n| n.as_string().ok().flatten())
                            .unwrap_or_default()
                    });
                    let on_click = move |_| {
                        navigate_student.run(student_id.clone());
                    };
                    // For now, we'll just display each student's first and last name.
                    // Now I'll add a container with some links. These links will just be appended
                    // to the end of the current path.

                    view! {
                        <div
                            class="flex flex-row p-1 border-y-1 border-gray-200 cursor-pointer hover:bg-yellow-100 transition-all duration-200"
                            on:click=on_click
                        >
                            <div class="pl-5 flex-1 text-lg text-left">{last_name}</div>
                            <div class="pr-5 flex-1 text-lg text-left">{first_name}</div>
                        </div>
                    }
                }
            />
        </div>
    }
}
