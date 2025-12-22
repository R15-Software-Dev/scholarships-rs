use super::UnauthenticatedPage;
use super::api::{
    CreateScholarshipInfo, CreateTestComparisons, get_comparison_info, get_scholarship_info,
};
use crate::common::ComparisonData;
use crate::common::{ExpandableInfo, ScholarshipFormParams};
use crate::components::{
    ActionButton, ChipsList, Loading, OutlinedTextField, Panel, RadioList, Row,
};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use leptos_router::hooks::use_params;
use traits::{AsReactive, ReactiveCapture};

/// # Scholarship Info Page
/// This page will handle creating or editing scholarship information given a specific
/// scholarship ID and scholarship provider subject number. The form itself will only
/// track a single scholarship (for now) and so selection of what scholarship will be edited
/// must be handled *before* navigation to this page.
///
/// In its current testing form, we'll be using a single fixed ID. This absolutely *MUST* be
/// changed before a full release.
#[component]
pub fn ScholarshipInfoPage() -> impl IntoView {
    // let scholarship_id = String::from("test-scholarship-id2");
    let url_params = use_params::<ScholarshipFormParams>();
    let scholarship_id = move || {
        url_params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id.clone())
            .unwrap_or_default()
    };

    let get_scholarship_action: Resource<ExpandableInfo> = Resource::new(
        move || scholarship_id().clone(),
        async |id| {
            get_scholarship_info(id.clone())
                .await
                // Note that this really should notify us of an error and then redirect or ask
                // the user to try again.
                .unwrap_or_else(|e| {
                    log!("Failed to get scholarship info: {:?}", e);
                    log!("Using default ExpandableInfo");
                    ExpandableInfo::new(id)
                })
        },
    );

    let get_comparison_info: Resource<Vec<ComparisonData>> = Resource::new(
        move || Option::<String>::None, // There's no input to this function.
        async |_| {
            get_comparison_info().await.unwrap_or_else(|e| {
                log!("Failed to get comparison info: {:?}", e);
                Vec::new()
            })
        },
    );

    let create_scholarship_action = ServerAction::<CreateScholarshipInfo>::new();
    let create_test_comps = ServerAction::<CreateTestComparisons>::new();

    // Remember that we only want to show the page after we've got authentication information
    // and if there's data for us to use.
    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                <Suspense fallback=Loading>
                    <Row>
                        <div class="flex flex-col flex-1" />
                        {move || Suspend::new(async move {
                            let response = get_scholarship_action.get();
                            let comparisons = get_comparison_info.get();

                            response
                                .zip(comparisons)
                                .map(|(response, comparison_list)| view! {
                                    <ScholarshipForm
                                        response=response
                                        comparison_list=comparison_list
                                        submit_action=create_scholarship_action
                                    />
                                })
                                .collect_view()
                        })}
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
                    </Row>
                </Suspense>
            </Authenticated>
        </AuthLoaded>
    }
}

#[component]
fn ScholarshipForm(
    response: ExpandableInfo,
    comparison_list: Vec<ComparisonData>,
    submit_action: ServerAction<CreateScholarshipInfo>,
) -> impl IntoView {
    let reactive_info = response.as_reactive();
    let elements_disabled = RwSignal::new(false);
    let result_msg = Signal::derive(move || {
        if submit_action.pending().get() {
            elements_disabled.set(true);
            "Sending request...".to_string()
        } else {
            elements_disabled.set(false);
            if let Some(result) = submit_action.value().get() {
                match result {
                    Ok(()) => "Request sent successfully".to_string(),
                    Err(err) => {
                        format!("An error occurred: {:?}", err)
                    }
                }
            } else {
                "".to_string()
            }
        }
    });

    // We'll collect the scholarship's name, num_awards, amount_per_award, total_awards,
    // fafsa_required, award_to, transcript_required, recipient_selection, essay_requirement (and prompt)
    // award_night_remarks

    view! {
        <Panel>
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
                    values=comparison_list
                        .iter()
                        .cloned()
                        .map(|comp| comp.id)
                        .collect()
                    displayed_text=comparison_list
                        .iter()
                        .cloned()
                        .map(|comp| comp.display_text)
                        .collect()
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
        </Panel>
    }
}
