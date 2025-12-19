use leptos::either::Either;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos_oidc::{AuthLoaded, Authenticated};
use leptos_router::hooks::{use_navigate, use_params};
use crate::common::{ExpandableInfo, ScholarshipFormParams, SubmitStatus, ValueType};
use crate::components::{ActionButton, Banner, ChipsList, Loading, OutlinedTextField, Panel, RadioList, Row};
use super::UnauthenticatedPage;
use traits::{AsReactive, ReactiveCapture};
use crate::common::ComparisonData;
use crate::pages::utils::get_user_claims;
use super::api::{get_comparison_info, get_provider_scholarships, get_scholarship_info, delete_provider_scholarship, CreateScholarshipInfo, CreateTestComparisons, RegisterScholarship};


/// # Scholarship Info Page
/// This page will handle creating or editing scholarship information given a specific 
/// scholarship ID and scholarship provider subject number. The form itself will only
/// track a single scholarship (for now) and so selection of what scholarship will be edited
/// must be handled *before* navigation to this page.
#[component]
pub fn ScholarshipInfoPage() -> impl IntoView {
    let url_params = use_params::<ScholarshipFormParams>();
    let scholarship_id = Memo::new(move |_| {
        url_params.read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
    });
    
    let create_test_comps = ServerAction::<CreateTestComparisons>::new();
    
    // Remember that we only want to show the page after we've got authentication information
    // and if there's data for us to use.
    view! {
        <Banner
            title="R15 Scholarships"
            logo="/PHS_Stacked_Acronym.png"
            path="/"
        />
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                <div class="flex flex-row align-top items-start">
                    <div class="flex flex-col flex-1" />
                    <ScholarshipList />
                    <Transition fallback=Loading>
                        <ScholarshipForm
                            scholarship_id=scholarship_id
                        />
                    </Transition>
                    <div class="hidden">
                        <Row>
                            <ActionButton
                                on:click=move |_| {
                                    create_test_comps.dispatch(CreateTestComparisons {});
                                }
                            >"Create test comparisons"</ActionButton>
                        </Row>
                    </div>
                    <div class="flex flex-col flex-1" />
                </div>
            </Authenticated>
        </AuthLoaded>
    }
}

#[component]
fn ScholarshipList() -> impl IntoView {
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
        move || provider_id.get(),
        async move |provider_id| {
            let Some(provider_id) = provider_id else {
                return Ok(Vec::new());
            };
            
            // Get the scholarships
            get_provider_scholarships(provider_id).await
        }
    );
    
    // Register scholarship creation/deletion actions
    let create_action = ServerAction::<RegisterScholarship>::new();
    
    // Button handlers
    let create_on_click = move |_| {
        // In the future, we'd actually like to create/show a dialog that gets the
        // desired name of the scholarship.
        // After the request is sent, we'll handle it in an effect.
        create_action.dispatch(RegisterScholarship {
            provider_id: provider_id.get().unwrap(),
            scholarship_name: "Testing Scholarship".to_string()
        });
    };
    
    // Register scholarship creation effect
    Effect::new(move || {
        // We want to wait for the action to complete first, but then find whether the request
        // succeeded.
        if let Some(Ok(_)) = create_action.value().get() {
            // Refetch and clear.
            scholarships.refetch();
            create_action.clear();
        }
    });
    
    view! {
        <div class="w-75">
            <Panel>
                <div class="pt-2">
                    <h2 class="text-xl font-bold text-center">"Scholarships"</h2>
                </div>
                <Transition fallback=Loading>
                    <div class="flex flex-col gap-2">
                        {move || {
                            scholarships.get().map(|result| {
                                match result {
                                    Ok(list) => {
                                        let views = list.iter().map(|entry| {
                                            view! { 
                                                <ScholarshipListEntry scholarship=entry.clone() 
                                                    on_delete=move || {
                                                        scholarships.refetch();
                                                    }/>
                                            }
                                        });
                                        Either::Left(views.collect_view())
                                    }
                                    Err(err) => {
                                        Either::Right(view! {
                                            <div>
                                                <p>"Failed to load scholarship list: "{err.to_string()}</p>
                                            </div>
                                        })
                                    }
                                }
                            }).collect_view()
                        }}
                    </div>
                    <div>
                        <div class="p-2 bg-red-800 rounded-md text-center text-white hover:bg-red-900 transition-all"
                            on:click=create_on_click>
                            "Create New"
                        </div>
                    </div>
                </Transition>
            </Panel>
        </div>
    }
}

#[component]
fn ScholarshipListEntry(
    #[prop()] scholarship: ExpandableInfo,
    #[prop(into)] on_delete: Callback<()>,
) -> impl IntoView {
    let navigate = use_navigate();
    
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
    
    let delete_click = {
        let subject = scholarship.subject.clone();
        move |_| {
            // Delete the scholarship.
            let subject = subject.clone();
            log!("Deleting scholarship with ID {:?}", subject);
            spawn_local(async move {
                match delete_provider_scholarship(
                    subject,
                    "".to_string()
                ).await {
                    Ok(_) => on_delete.run(()),
                    _ => {}
                }
            });
        }
    };
    
    view! {
        <div class="flex flex-col rounded-md border-light inset-shadow-xs shadow-md my-2 transition-all hover:shadow-lg/33">
            <div class="p-2 text-center">
                <span>{name}</span>
            </div>
            <div class="flex flex-row">
                <div class="p-2 flex-1 bg-red-800 border-r border-white rounded-bl-md text-white text-center
                    hover:bg-red-900 cursor-pointer transition-all"
                    on:click=edit_click
                >
                    "Edit"
                </div>
                <div class="p-2 flex-1 bg-red-800 border-l border-white rounded-br-md text-white text-center
                    hover:bg-red-900 cursor-pointer transition-all"
                    on:click=delete_click
                >
                    "Delete"
                </div>
            </div>
        </div>
    }
}

#[component]
fn ScholarshipForm(
    #[prop(into)] scholarship_id: Signal<Option<String>>
) -> impl IntoView {
    let submit_status = RwSignal::new(SubmitStatus::Idle);
    let elements_disabled = Signal::derive(move || {
        match submit_status.get() {
            SubmitStatus::Sending => true,
            _ => false
        }
    });
    let result_msg = Signal::derive(move || {
        match submit_status.get() {
            SubmitStatus::Idle => "".into(),
            SubmitStatus::Sending => "Sending submission...".into(),
            SubmitStatus::Success => "Success".into(),
            SubmitStatus::Error(msg) => format!("Error: {}", msg),
        }
    });

    let submit_action = ServerAction::<CreateScholarshipInfo>::new();
    
    Effect::new(move || {
        if submit_action.pending().get() {
            submit_status.set(SubmitStatus::Sending);
            return;
        }
        
        if let Some(result) = submit_action.value().get() {
            match result {
                Ok(()) => submit_status.set(SubmitStatus::Success),
                Err(err) => submit_status.set(SubmitStatus::Error(err.to_string()))
            }
        }
    });
    
    let comparison_list: Resource<Vec<ComparisonData>> = Resource::new(
        move || scholarship_id.get(),  // There's no input to this function.
        async move |_| {
            get_comparison_info().await
                .unwrap_or_else(|e| {
                    log!("Failed to get comparison info: {:?}", e);
                    Vec::new()
                })
        }
    );

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
    
    let comparison_ids = Memo::new(move |_| {
        comparison_list.get().map(|list| list.iter()
            .map(|comp| comp.clone().id)
            .collect::<Vec<String>>()
        ).unwrap_or_default()
    });
    
    let comparison_text = Memo::new(move |_| {
        comparison_list.get().map(|list| list.iter()
            .map(|comp| comp.clone().display_text)
            .collect::<Vec<String>>()
        ).unwrap_or_default()
    });

    // We'll collect the scholarship's name, num_awards, amount_per_award, total_awards,
    // fafsa_required, award_to, transcript_required, recipient_selection, essay_requirement (and prompt)
    // award_night_remarks
    
    view! {
        <Panel>
            <Show
                when=move || scholarship_id.get().is_some()
                fallback=|| view! { <p>"Choose a scholarship from the right, or create a new one."</p> }
            >
                <Suspense fallback=Loading>
                    {move || {
                        scholarship_info.get()
                            .map(|res_scholarship| {
                                match res_scholarship {
                                    Ok(scholarship) => {
                                        let reactive_info = scholarship.as_reactive();
                                    
                                        Either::Left(view! {
                                            <Row>
                                                <h1 class="text-3xl font-bold">
                                                    "Scholarship Info Form (test)"
                                                </h1>
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="Scholarship Name"
                                                    placeholder="Example Scholarship Name"
                                                    disabled=elements_disabled
                                                    data_member="name"
                                                    data_map=reactive_info.data
                                                />
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="Amount per award: (if multiple, enter the highest value)"
                                                    placeholder="Enter an amount..."
                                                    disabled=elements_disabled
                                                    data_member="amount_per_award"
                                                    data_map=reactive_info.data
                                                    input_type="number"
                                                />
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="Total number of awards:"
                                                    placeholder="Enter an amount..."
                                                    disabled=elements_disabled
                                                    data_member="num_awards"
                                                    data_map=reactive_info.data
                                                    input_type="number"
                                                />
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="Total amount of all awards:"
                                                    placeholder="Enter an amount..."
                                                    disabled=elements_disabled
                                                    data_member="total_awards"
                                                    data_map=reactive_info.data
                                                    input_type="number"
                                                />
                                            </Row>
                                            <Row>
                                                <RadioList
                                                    data_member="fafsa_required"
                                                    data_map=reactive_info.data
                                                    items=vec!["Yes".to_string(), "No".to_string()]
                                                    disabled=elements_disabled
                                                    label="Do you require student financial information?"
                                                />
                                            </Row>
                                            <Row>
                                                <RadioList
                                                    data_member="transcript_required"
                                                    data_map=reactive_info.data
                                                    items=vec!["Yes".to_string(), "No".to_string()]
                                                    disabled=elements_disabled
                                                    label="Do you require a student transcript?"
                                                />
                                            </Row>
                                            <Row>
                                                <RadioList
                                                    data_member="award_to"
                                                    data_map=reactive_info.data
                                                    items=vec!["School".to_string(), "Student".to_string()]
                                                    disabled=elements_disabled
                                                    label="Will the award be made to the school or the student?"
                                                />
                                            </Row>
                                            <Row>
                                                <RadioList
                                                    data_member="essay_required"
                                                    data_map=reactive_info.data
                                                    items=vec!["Yes".to_string(), "No".to_string()]
                                                    disabled=elements_disabled
                                                    label="Do you require a student essay?"
                                                />
                                            </Row>
                                            <Row>
                                                <RadioList
                                                    data_member="essay_prompt"
                                                    data_map=reactive_info.data
                                                    items=vec!["Test 1", "Test 2", "Test 3"].iter().map(|s| s.to_string()).collect()
                                                    disabled=elements_disabled
                                                    label="If so, select a prompt from the list below."
                                                />
                                            </Row>
                                            <Row>
                                                <ChipsList
                                                    label="Scholarship Requirements"
                                                    data_member="requirements"
                                                    data_map=reactive_info.data
                                                    values=comparison_ids
                                                    displayed_text=comparison_text
                                                />
                                            </Row>
                                            <Row>
                                                <ActionButton
                                                    disabled=elements_disabled
                                                    on:click=move |_| {
                                                        let captured = reactive_info.capture();
                                                        log!("Map values: {:?}", captured);
                                                        submit_action.dispatch(CreateScholarshipInfo {
                                                            info: captured
                                                        });
                                                    }
                                                >"Submit"</ActionButton>
                                            </Row>
                                            <Row>
                                                <p>{result_msg}</p>
                                            </Row>
                                        })
                                    }
                                    Err(err) => {
                                        Either::Right(view! {
                                            <p>"Error while getting scholarship: "{err.to_string()}</p>
                                        })
                                    }
                                }
                            }).collect_view()
                    }}
                </Suspense>
            </Show>
        </Panel>
    }
}
