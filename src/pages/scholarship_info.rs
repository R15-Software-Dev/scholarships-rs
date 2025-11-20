#[cfg(feature = "ssr")]
use super::utils::server_utils::create_dynamo_client;

#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{
    error::ProvideErrorMetadata,
    types::AttributeValue,
};

use leptos::logging::log;
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use crate::common::{ComparisonType, ExpandableInfo, NumberComparison, ValueType};
use crate::components::{ActionButton, ChipsList, Loading, OutlinedTextField, Panel, Row};
use super::UnauthenticatedPage;
use traits::{AsReactive, ReactiveCapture};
use crate::common::ComparisonData;

#[server(GetScholarshipInfo, endpoint = "/scholarship/info/get")]
pub async fn get_scholarship_info(id: String) -> Result<ExpandableInfo, ServerFnError> {    
    let client = create_dynamo_client().await;
    
    // Perform the operation - we just want to return all data that's contained in this entry,
    // or just return an empty ExpandableInfo struct.
    log!("Getting scholarship info using id {:?}", id);
    match client
        .get_item()
        .table_name("leptos-scholarship-test")
        .key("subject", AttributeValue::S(id.clone()))
        .send()
        .await 
    {
        Ok(output) => {
            if let Some(item) = output.item {
                log!("Found output from API: {:?}", item);
                Ok(serde_dynamo::from_item(item)?)
            } else {
                log!("Couldn't find any values, returning default struct.");
                Ok(ExpandableInfo::new(id.clone()))
            }
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server(CreateScholarshipInfo, endpoint = "/scholarship/info/create")]
pub async fn create_scholarship_info(info: ExpandableInfo) -> Result<(), ServerFnError> {
    use super::utils::server_utils::create_dynamo_client;
    use aws_sdk_dynamodb::error::ProvideErrorMetadata;
    
    let client = create_dynamo_client().await;
    
    log!("Creating or updating scholarship with ID {:?}", info.subject);
    
    match client
        .put_item()
        .table_name("leptos-scholarship-test")
        .set_item(Some(serde_dynamo::to_item(&info)?))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server]
async fn get_comparison_info() -> Result<Vec<ComparisonData>, ServerFnError> {
    let client = create_dynamo_client().await;
    
    log!("Getting all comparisons from the database");
    
    // Query the database for all comparisons. The client is only going to use the
    // id and display text, but we'll return the whole thing.
    match client
        .scan()
        .table_name("leptos-comparison-test")
        .send()
        .await
    {
        Ok(output) => {
            if let Some(items) = output.items {
                log!("Found comparisons from API: {:?}", items);
                Ok(serde_dynamo::from_items(items)?)
            } else {
                Ok(vec![])
            }
        },
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server(CreateTestComparisons, endpoint = "/comparisons/create-test")]
async fn create_test_comparisons() -> Result<(), ServerFnError> {
    let client = create_dynamo_client().await;
    
    log!("Creating test comparisons");
    
    let test_comp = ComparisonData::new(
        "comp1",
        "unweighted_gpa",
        ComparisonType::Number(NumberComparison::GreaterThanOrEqual),
        ValueType::Number(Some(3.2.to_string())),
        "Testing",
        "GPA > 3.2"
    );
    
    let _comp_list = vec![
        ComparisonData::new(
            "comp1",
            "unweighted_gpa",
            ComparisonType::Number(NumberComparison::GreaterThanOrEqual),
            ValueType::Number(Some(3.2.to_string())),
            "Testing",
            "GPA > 3.2"
        )
    ];
    
    match client
        .put_item()
        .table_name("leptos-comparison-test")
        .set_item(Some(serde_dynamo::to_item(&test_comp)?))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

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
    let scholarship_id = String::from("test-scholarship-id");
    
    let get_scholarship_action: Resource<ExpandableInfo> = Resource::new(
        move || scholarship_id.clone(),
        async |id| {
            get_scholarship_info(id).await
                // Note that this really should notify us of an error and then redirect or ask
                // the user to try again.
                .unwrap_or_else(|e| {
                    log!("Failed to get scholarship info: {:?}", e);
                    log!("Using default ExpandableInfo");
                    ExpandableInfo::new("test-scholarship-id")
                })
        }
    );
    
    let get_comparison_info: Resource<Vec<ComparisonData>> = Resource::new(
        move || Option::<String>::None,  // There's no input to this function.
        async |_| {
            get_comparison_info().await
                .unwrap_or_else(|e| {
                    log!("Failed to get comparison info: {:?}", e);
                    Vec::new()
                })
        }
    );
    
    let create_scholarship_action = ServerAction::<CreateScholarshipInfo>::new();
    let create_test_comps = ServerAction::<CreateTestComparisons>::new();
    
    // Remember that we only want to show the page after we've got authentication information
    // and if there's data for us to use.
    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                <Suspense fallback=Loading>
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
                    <Row>
                        <ActionButton
                            on:click=move |_| {
                                create_test_comps.dispatch(CreateTestComparisons {});
                            }
                        >"Create test comparisons"</ActionButton>
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
    submit_action: ServerAction<CreateScholarshipInfo>
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
                <ChipsList
                    label="Scholarship Requirements"
                    data_member="requirements"
                    data_map=reactive_info.data
                    items=comparison_list
                        .into_iter()
                        .map(|comp| comp.id)
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
