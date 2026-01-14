use std::collections::HashMap;
use leptos::either::Either;
use leptos::html::Dialog;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use leptos_router::hooks::{use_navigate, use_params};
use crate::common::{ExpandableInfo, ScholarshipFormParams, SubmitStatus, ValueType};
use crate::components::{ActionButton, Banner, ChipsList, Loading, OutlinedTextField, Panel, RadioList, Row, TextFieldType, Toast, ToastContext, ToastList, ValidatedForm};
use super::UnauthenticatedPage;
use crate::pages::utils::get_user_claims;
use super::api::{get_provider_scholarships, get_scholarship_info, CreateScholarshipInfo, RegisterScholarship, DeleteProviderScholarship, get_comparisons_categorized, CreateTestComparisons};


/// # Scholarship Info Page
/// This page will handle creating or editing scholarship information given a specific 
/// scholarship ID and scholarship provider subject number. The form itself will only
/// track a single scholarship (for now) and so selection of what scholarship will be edited
/// must be handled *before* navigation to this page.
#[component]
pub fn ScholarshipInfoPage() -> impl IntoView {
    // Changes to this signal update the scholarship list.
    let list_refresh = RwSignal::new(0);

    let url_params = use_params::<ScholarshipFormParams>();
    let scholarship_id = Memo::new(move |_| {
        url_params.read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
    });
    
    // let create_test_comps = ServerAction::<CreateTestComparisons>::new();
    
    let on_delete = move || {
        let navigate = use_navigate();
        navigate("/providers/scholarships", Default::default());
    };
    
    // Remember that we only want to show the page after we've got authentication information
    // and if there's data for us to use.
    view! {
        <Banner title="R15 Scholarships" logo="/PHS_Stacked_Acronym.png" path="/" />
        <ToastList>
            <AuthLoaded fallback=Loading>
                <Authenticated unauthenticated=UnauthenticatedPage>
                    <div class="flex flex-row align-top items-start">
                        <div class="flex flex-col flex-1" />
                        <ScholarshipList refresh_token=list_refresh on_delete=on_delete />
                        <Transition fallback=Loading>
                            <ScholarshipForm
                                scholarship_id=scholarship_id
                                on_submit=move || {
                                    list_refresh.update(|n| *n += 1);
                                }
                            />
                        </Transition>
                        <div class="flex flex-col flex-1" />
                    </div>
                </Authenticated>
            </AuthLoaded>
        </ToastList>
    }
}

/// # Scholarship List Component
/// 
/// This component should always be wrapped in an Authenticated component. Assuming that a provider
/// is authenticated, this component will get all the scholarships that this provider owns and 
/// will display them in a list, giving them the option to edit, delete, or create new scholarships.
/// 
/// Example usage:
/// ```
/// let refresh = RwSignal::new(0);
/// 
/// view! {
///     <AuthLoaded>
///         <Authenticated>
///             <ScholarshipList 
///                 refresh_token=refresh
///                 on_delete=move || log!("This is run when an item is deleted on the server.")
///             />
///         </Authenticated>
///     </AuthLoaded>
/// }
/// ```
#[component]
fn ScholarshipList(
    /// A signal that refreshes the list when changed. Most times, we don't need to worry about the
    /// user reloading our page 2 billion times.
    #[prop()] refresh_token: RwSignal<i32>,
    /// A callback that is run on successful server-side deletion of an item.
    #[prop(into)] on_delete: Callback<()>
) -> impl IntoView {
    // List state
    let pending_delete = RwSignal::new(None::<ExpandableInfo>);
    let delete_dialog_ref = NodeRef::<Dialog>::new();

    // Get scholarship provider information from authentication. This element will not display
    // unless that information is available.
    let user_claims = get_user_claims();
    let provider_id = Memo::new(move |_| 
        user_claims.get()
            .as_ref()
            .map(|info| info.claims.subject.clone())
    );
    
    // Register scholarship server resource
    let scholarships = Resource::new(
        move || {
            (refresh_token.get(), provider_id.get())
        },
        |(_, provider_id)| async move {
            log!("Fetching from scholarship API.");
            let Some(provider_id) = provider_id else {
                return Ok(Vec::new());
            };
            
            // Get the scholarships
            get_provider_scholarships(provider_id).await
        }
    );
    
    // Register scholarship creation/deletion actions
    let create_action = ServerAction::<RegisterScholarship>::new();
    let delete_action = ServerAction::<DeleteProviderScholarship>::new();
    
    // Button handlers
    let create_on_click = move |_| {
        create_action.dispatch(RegisterScholarship {
            provider_id: provider_id.get().unwrap(),
            scholarship_name: "Testing Scholarship".to_string()
        });
    };

    let on_item_delete = move |s| {
        pending_delete.set(Some(s));
    };
    
    // Register scholarship creation effect
    Effect::new(move || {
        // We want to wait for the action to complete first, but then find whether the request
        // succeeded.
        if let Some(Ok(_)) = create_action.value().get() {
            // Refetch and clear. We could alter this slightly to get the new element and
            // update the list on the client side only (no refetch). We also want to set the name
            // of the new scholarship before it's created on the server.
            scholarships.refetch();
            create_action.clear();
        }
    });

    // Register scholarship deletion effects
    let modal_disabled = Signal::derive(move || delete_action.pending().get());
    Effect::new(move || {
        if let Some(Ok(_)) = delete_action.value().get() {
            scholarships.refetch();
            pending_delete.set(None);
            delete_action.clear();
            on_delete.run(());
        }
    });

    // Register scholarship pending delete effects
    Effect::new(move || {
        if let Some(dialog) = delete_dialog_ref.get() {
            if pending_delete.get().is_some() {
                // Find the modal dialog and show it. Ask for the user's confirmation. Afterward,
                // take the correct action.
                dialog.show_modal()
                    .expect("Couldn't show dialog. Is it already open?");
            } else {
                dialog.close();
            }
        }
    });
    
    view! {
        <dialog node_ref=delete_dialog_ref class="m-auto p-2">
            <h2 class="text-2xl font-bold">"Confirm Deletion"</h2>
            <p>
                "Are you sure you want to delete "
                {move || {
                    pending_delete
                        .get()
                        .map(|s| {
                            s.data
                                .get("name")
                                .unwrap_or(&ValueType::String(None))
                                .as_string()
                                .unwrap_or_default()
                                .unwrap_or("<no name found>".to_string())
                        })
                }} "?"
            </p>
            <ActionButton
                on:click=move |_| {
                    let subject = pending_delete
                        .get()
                        .map(|s| s.subject.clone())
                        .unwrap_or_default();
                    log!("Deleting scholarship with subject {}", subject);
                    delete_action
                        .dispatch(DeleteProviderScholarship {
                            scholarship_id: subject,
                            provider_id: provider_id.get().unwrap_or_default(),
                        });
                }
                disabled=modal_disabled
            >
                "Confirm"
            </ActionButton>
            <ActionButton on:click=move |_| pending_delete.set(None)>"Cancel"</ActionButton>
        </dialog>
        <div class="w-75">
            <Panel>
                <div class="pt-2">
                    <h2 class="text-xl font-bold text-center">"Scholarships"</h2>
                </div>
                <Transition fallback=Loading>
                    <div class="flex flex-col gap-2">
                        {move || {
                            scholarships
                                .get()
                                .map(|result| {
                                    match result {
                                        Ok(list) => {
                                            let views = list
                                                .iter()
                                                .map(|entry| {
                                                    view! {
                                                        <ScholarshipListEntry
                                                            scholarship=entry.clone()
                                                            on_delete=Callback::new(on_item_delete)
                                                        />
                                                    }
                                                });
                                            Either::Left(views.collect_view())
                                        }
                                        Err(err) => {
                                            Either::Right(
                                                view! {
                                                    <div>
                                                        <p>"Failed to load scholarship list: "{err.to_string()}</p>
                                                    </div>
                                                },
                                            )
                                        }
                                    }
                                })
                                .collect_view()
                        }}
                    </div>
                    <div>
                        <div
                            class="p-2 bg-red-800 rounded-md text-center text-white hover:bg-red-900 transition-all cursor-pointer"
                            on:click=create_on_click
                        >
                            "Create New"
                        </div>
                    </div>
                </Transition>
            </Panel>
        </div>
    }
}

/// # Scholarship List Entry Component
/// 
/// Displays a single scholarship within the [`ScholarshipList`] component.
/// 
/// Example usage (within the [`ScholarshipList`]):
/// ```
/// view! {
///     <ScholarshipListEntry
///         scholarship= /* An ExpandableInfo object containing scholarship info */
///         on_delete=move |_| log!("This is a callback on successful deletion.")
///     />
/// }
/// ```
#[component]
fn ScholarshipListEntry(
    /// The scholarship information. This element will attempt to get the name of the scholarship
    /// using its `name` field, and also utilizes the `subject` field.
    #[prop()] scholarship: ExpandableInfo,
    /// A [`Callback<ExpandableInfo>`] that runs upon successful deletion of the scholarship.
    #[prop(into)] on_delete: Callback<ExpandableInfo, ()>,
) -> impl IntoView {
    let navigate = use_navigate();
    let user_claims = get_user_claims();
    let _user_id = Memo::new(move |_| {
        user_claims.get()
            .map(|info| info.claims.subject.clone())
    });
    
    // Get the values that we need out of the info. We just need the subject and the name of 
    // the scholarship.
    let name = scholarship.data.get("name")
        .unwrap_or(&ValueType::String(None))
        .as_string().unwrap_or_default()
        .unwrap_or("<no name found>".to_string());
    
    // Button click handlers
    let edit_click = {
        let subject = scholarship.subject.clone();
        move |_| {
            let path = format!("/providers/scholarships/{}", subject);
            navigate(path.as_str(), Default::default());
        }
    };
    
    view! {
        <div class="flex flex-col rounded-md border-light inset-shadow-xs shadow-md my-2 transition-all hover:shadow-lg/33">
            <div class="p-2 text-center">
                <span>{name}</span>
            </div>
            <div class="flex flex-row">
                <div
                    class="p-2 flex-1 bg-red-800 border-r border-white rounded-bl-md text-white text-center
                    hover:bg-red-900 cursor-pointer transition-all"
                    on:click=edit_click
                >
                    "Edit"
                </div>
                <div
                    class="p-2 flex-1 bg-red-800 border-l border-white rounded-br-md text-white text-center
                    hover:bg-red-900 cursor-pointer transition-all"
                    on:click=move |_| on_delete.run(scholarship.clone())
                >
                    "Delete"
                </div>
            </div>
        </div>
    }
}

/// # Scholarship Edit Form Component
///
/// Displays all information about a scholarship and allows the provider to edit that information.
/// All information is only saved when the submit button is clicked.
///
/// The component will call the `on_submit` callback when the API returns an `Ok` result. In the
/// event that the API call fails, it will not call anything. This is mainly used to allow updates
/// to neighboring components from the parent.
///
/// Example usage:
/// ```
/// view! {
///     <ScholarshipForm
///         scholarship_id="SomeScholarshipID"
///         on_submit=move || log!("This is a callback!")
///     />
/// }
/// ```
#[component]
fn ScholarshipForm(
    /// The ID of the scholarship to edit. The form will handle getting all information about the
    /// scholarship and submitting new information.
    #[prop(into)] scholarship_id: Signal<Option<String>>,
    /// A callback that is run when the form successfully submits new data. In the event that the
    /// submission fails, this function is not run.
    #[prop(into)] on_submit: Callback<()>
) -> impl IntoView {
    let submit_action = ServerAction::<CreateScholarshipInfo>::new();

    let submit_status = RwSignal::new(SubmitStatus::Idle);
    let elements_disabled = Signal::derive(move || {
        matches!(submit_action.pending().get(), true)
    });

    Effect::new(move || {
        if submit_action.pending().get() {
            submit_status.set(SubmitStatus::Sending);
            return;
        }
        
        if let Some(result) = submit_action.value().get() {
            match result {
                Ok(()) => {
                    submit_status.set(SubmitStatus::Success);
                    on_submit.run(());
                },
                Err(_err) => submit_status.set(SubmitStatus::Error(()))
            }
        }
    });

    let comparison_lists = Resource::new(
        move || scholarship_id.get().is_some(),
        async move |_| {
            get_comparisons_categorized().await
        }
    );

    Effect::new(move || {
        if let Some(Ok(map)) = comparison_lists.get() {
            log!("Available categories: {:?}", map.keys());
        }
    });

    let scholarship_info = Resource::new(
        move || scholarship_id.get(),
        async move |opt_id| {
            if let Some(id) = opt_id {
                get_scholarship_info(id).await
            } else {
                Err(ServerFnError::new("Cannot get from server with no ID."))
            }
        }
    );

    let form_data = RwSignal::new(HashMap::new());
    let chips_data = RwSignal::new(HashMap::new());
    
    Effect::new(move || {
        if let Some(Ok(scholarship)) = scholarship_info.get() {
            // Get the default chips data.
            let chips_default = if let Some(ValueType::Map(Some(map))) = scholarship.data.get("requirements") {
                log!("Found requirements map: {:?}", map);
                map.clone()
            } else {
                log!("Couldn't find requirements map in scholarship info. Using empty map.");
                HashMap::new()
            };

            chips_data.set(chips_default);
            form_data.set(scholarship.data);
        }
    });

    let get_comp_category = move |category: &str| {
        if let Some(Ok(map)) = comparison_lists.get() {
            map.get(category).unwrap_or(&Vec::new()).clone()
        } else {
            Vec::new()
        }
    };

    let get_comp_ids = move |category: &str| {
        get_comp_category(category)
            .into_iter()
            .map(|comp| comp.id)
            .collect::<Vec<String>>()
    };

    let get_comp_text = move |category: &str| {
        get_comp_category(category)
            .into_iter()
            .map(|comp| comp.display_text)
            .collect::<Vec<String>>()
    };

    // Get comparison lists.
    let sports_ids = Signal::derive(move || get_comp_ids("Sports Participation"));
    let sports_text = Signal::derive(move || get_comp_text("Sports Participation"));

    let misc_ids = Signal::derive(move || get_comp_ids("Additional Eligibility Factors"));
    let misc_text = Signal::derive(move || get_comp_text("Additional Eligibility Factors"));

    let comm_ids = Signal::derive(move || get_comp_ids("Community Service"));
    let comm_text = Signal::derive(move || get_comp_text("Community Service"));

    let res_ids = Signal::derive(move || get_comp_ids("Residency"));
    let res_text = Signal::derive(move || get_comp_text("Residency"));

    let gpa_ids = Signal::derive(move || get_comp_ids("GPA Limits"));
    let gpa_text = Signal::derive(move || get_comp_text("GPA Limits"));

    let major_ids = Signal::derive(move || get_comp_ids("Majors"));
    let major_text = Signal::derive(move || get_comp_text("Majors"));

    let mut toasts = expect_context::<ToastContext>();
    
    let on_submit = move |_| {
        let mut info = ExpandableInfo::new(scholarship_id.get().unwrap_or_default());
        info.data = form_data.get();
        info.data.insert("requirements".to_string(), ValueType::Map(Some(chips_data.get())));

        log!("Map values: {:?}", info.data);
        submit_action.dispatch(CreateScholarshipInfo {
            info
        });
    };
    
    Effect::new(move || {
        match submit_action.value().get() {
            Some(Ok(_)) => toasts.toast(
                Toast::new()
                    .id(uuid::Uuid::new_v4())
                    .header("Submission Successful")
                    .msg("You can go back or continue editing your responses.")
            ),
            Some(Err(err)) => toasts.toast(
                Toast::new()
                    .id(uuid::Uuid::new_v4())
                    .header("Submission Failed")
                    .msg(err.to_string())
            ),
            _ => {}
        }

        submit_action.clear();
    });

    let create_comps = ServerAction::<CreateTestComparisons>::new();

    view! {
        <Panel>
            <Show
                when=move || scholarship_id.get().is_some()
                fallback=|| {
                    view! { <p>"Choose a scholarship from the right, or create a new one."</p> }
                }
            >
                <Suspense fallback=Loading>
                    <ValidatedForm
                        on_submit=Callback::new(on_submit)
                        title="Scholarship Info Form"
                        description="Create or edit a scholarship. Input all information and click Submit."
                    >
                        {move || {
                            scholarship_info
                                .get()
                                .map(|res_scholarship| {
                                    match res_scholarship {
                                        Ok(_) => {
                                            Either::Left(
                                                view! {
                                                    <Row>
                                                        <OutlinedTextField
                                                            label="Scholarship Name"
                                                            placeholder="Example Scholarship Name"
                                                            disabled=elements_disabled
                                                            data_member="name"
                                                            data_map=form_data
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <OutlinedTextField
                                                            label="Amount per award: (if multiple, enter the highest value)"
                                                            placeholder="Enter an amount..."
                                                            disabled=elements_disabled
                                                            data_member="amount_per_award"
                                                            data_map=form_data
                                                            input_type=TextFieldType::Number
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <OutlinedTextField
                                                            label="Total number of awards:"
                                                            placeholder="Enter an amount..."
                                                            disabled=elements_disabled
                                                            data_member="num_awards"
                                                            data_map=form_data
                                                            input_type=TextFieldType::Number
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <OutlinedTextField
                                                            label="Total amount of all award(s): (estimates are acceptable)"
                                                            placeholder="Enter an amount..."
                                                            disabled=elements_disabled
                                                            data_member="total_awards"
                                                            data_map=form_data
                                                            input_type=TextFieldType::Number
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <RadioList
                                                            data_member="fafsa_required"
                                                            data_map=form_data
                                                            items=vec!["Yes".to_string(), "No".to_string()]
                                                            disabled=elements_disabled
                                                            label="Do you require student FAFSA information?"
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <RadioList
                                                            data_member="transcript_required"
                                                            data_map=form_data
                                                            items=vec!["Yes".to_string(), "No".to_string()]
                                                            disabled=elements_disabled
                                                            label="Do you require a student transcript?"
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <RadioList
                                                            data_member="award_to"
                                                            data_map=form_data
                                                            items=vec!["School".to_string(), "Student".to_string()]
                                                            disabled=elements_disabled
                                                            label="Will the award be made to the school or the student?"
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <RadioList
                                                            data_member="essay_required"
                                                            data_map=form_data
                                                            items=vec!["Yes".to_string(), "No".to_string()]
                                                            disabled=elements_disabled
                                                            label="Do you require a student essay?"
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <RadioList
                                                            data_member="essay_prompt"
                                                            data_map=form_data
                                                            items=vec!["Test 1", "Test 2", "Test 3"]
                                                                .iter()
                                                                .map(|s| s.to_string())
                                                                .collect()
                                                            disabled=elements_disabled
                                                            label="If so, select a prompt from the list below."
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <ChipsList
                                                            label="GPA Requirements"
                                                            data_member="gpa"
                                                            data_map=chips_data
                                                            values=gpa_ids
                                                            displayed_text=gpa_text
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <ChipsList
                                                            label="Sports Participation"
                                                            data_member="sports_participation"
                                                            data_map=chips_data
                                                            values=sports_ids
                                                            displayed_text=sports_text
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <ChipsList
                                                            label="Community Service"
                                                            data_member="community_service"
                                                            data_map=chips_data
                                                            values=comm_ids
                                                            displayed_text=comm_text
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <ChipsList
                                                            label="Residency"
                                                            data_member="residency"
                                                            data_map=chips_data
                                                            values=res_ids
                                                            displayed_text=res_text
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <ChipsList
                                                            label="Majors"
                                                            data_member="major"
                                                            data_map=chips_data
                                                            values=major_ids
                                                            displayed_text=major_text
                                                        />
                                                    </Row>
                                                    <Row>
                                                        <ChipsList
                                                            label="Additional Eligibility Factors"
                                                            data_member="misc"
                                                            data_map=chips_data
                                                            values=misc_ids
                                                            displayed_text=misc_text
                                                        />
                                                    </Row>
                                                },
                                            )
                                        }
                                        Err(err) => {
                                            Either::Right(
                                                view! {
                                                    <p>"Error while getting scholarship: "{err.to_string()}</p>
                                                },
                                            )
                                        }
                                    }
                                })
                                .collect_view()
                        }}
                    </ValidatedForm>
                    <ActionButton on:click=move |_| {
                        create_comps.dispatch(CreateTestComparisons {});
                    }>"Create Comparisons"</ActionButton>
                </Suspense>
            </Show>
        </Panel>
    }.into_any()
}
