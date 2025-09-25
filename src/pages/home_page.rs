use std::collections::HashMap;
// Server dependencies
#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{Client, error::ProvideErrorMetadata, types::AttributeValue};

#[cfg(feature = "ssr")]
use serde_dynamo::{from_item, to_item};

use crate::app::Unauthenticated;
use crate::common::{ExpandableInfo, StudentInfo, UserClaims};
use crate::components::{ActionButton, Loading, OutlinedTextField, Panel, RadioList, Row, Select};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_oidc::{Algorithm, AuthLoaded, AuthSignal, Authenticated};
use reactive_stores::Store;
use traits::{AsReactive, ReactiveCapture};

/// # Get Student Info
/// Gets a student's information given their `subject`.
/// 
/// All information is found by using a `GetItemCommand` in the student application DynamoDB table.
#[server(GetSubmission, endpoint = "/get-submission")]
pub async fn get_submission(id: String) -> Result<StudentInfo, ServerFnError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    console_log(format!("Getting values from API using username {}", id).as_str());

    // Gets the item from the database. It only returns an error if the database
    // experiences an error - not if the item is not found.
    match dbclient
        .get_item()
        .table_name("student-applications")
        .key("Email", AttributeValue::S(id.clone()))
        .send()
        .await
    {
        Ok(output) => {
            if let Some(item) = output.item {
                console_log(format!("Found item from API: {item:?}").as_str());
                Ok(from_item(item)?)
            } else {
                console_log("Couldn't find entry, returning default.");
                let mut info = StudentInfo::default();
                info.Email = id; // Set the correct subject
                Ok(info)
            }
        }
        Err(err) => {
            let msg = err.message().unwrap_or("Unknown error");
            Err(ServerFnError::new(msg))
        }
    }
}

/// # Create Sample Submission
/// Creates a submission given a full `StudentInfo` struct.
/// 
/// All information is stored using a `PutItemCommand` in the student application DynamoDB table.
#[server(CreateSampleSubmission, endpoint = "/create-sample-submission")]
pub async fn create_sample_submission(student_info: StudentInfo) -> Result<(), ServerFnError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    // The put_item action can create or update an item in a DynamoDB table.
    // It will completely replace any existing item with the same primary key,
    // which is the desired behavior.
    let item = to_item(student_info.clone())?;
    console_log(format!("Arguments for sample submission: {:?}", item).as_str());
    match dbclient
        .put_item()
        .table_name("student-applications")
        .set_item(Some(item))
        .send()
        .await
    {
        Ok(_) => {
            // The student's information was successfully created
            console_log(format!("Set up student information: {:?}", student_info).as_str());
            Ok(())
        }
        Err(err) => {
            // There was an error while creating the student's information
            let msg = err.message().unwrap_or("An unknown error occurred.");
            console_log(format!("Error creating sample submission: {}", msg).as_str());
            Err(ServerFnError::new(msg))
        }
    }
}

#[server(LogExpandableInfo, endpoint = "/log-expandable")]
pub async fn log_expandable(info: ExpandableInfo) -> Result<(), ServerFnError> {
    console_log(format!("Was given info: {:?}", info).as_str());
    // Manually specify the type here since we're not able to infer the type later.
    let item: HashMap<String, AttributeValue> = to_item(info.clone())?;
    console_log(format!("Converted into a DynamoDB item: {:?}", item).as_str());
    
    Ok(())
}

/// The main home page component. Contains a simple contact form.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    let auth = use_context::<AuthSignal>().expect("Couldn't find user information.");
    let user_claims = Signal::derive(move || {
        auth.with(|auth| {
            auth.authenticated().and_then(|data| {
                data.decoded_access_token::<UserClaims>(Algorithm::RS256, &["account"])
            })
        })
    });

    // Note that the value passed in MUST be equatable.
    // We get/unwrap the value repeatedly until we get a simple string value, then clone it so that
    // we don't lose access to it in the future, should we need it again.
    let server_resource: Resource<StudentInfo> = Resource::new(
        move || user_claims.get().map(|claim| claim.claims.subject.clone()),
        async |opt_username| match opt_username {
            Some(username) => get_submission(username).await.unwrap_or_else(|e| {
                console_log(e.to_string().as_str());
                StudentInfo::default()
            }),
            None => StudentInfo::default(),
        },
    );
    let submit_action = ServerAction::<CreateSampleSubmission>::new();
    let log_action = ServerAction::<LogExpandableInfo>::new();

    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=Unauthenticated>
                // Replace this fallback with a real loading screen.
                <Suspense fallback=Loading>
                    {move || {
                        server_resource
                            .get()
                            .map(|submission| {
                                let store_info = Store::new(submission.clone());
                                let expandable = ExpandableInfo::new("email".into());
                                let expandable_react = expandable.as_reactive();
                                let reactive_info = submission.as_reactive();
                                let elements_disabled = RwSignal::new(false);
                                let result_msg = Signal::derive(move || {
                                    // Eventually, this function will show and hide a loading symbol
                                    // and a success checkmark.
                                    if submit_action.pending().get() {
                                        elements_disabled.set(true);
                                        "Sending request...".to_owned()
                                    } else {
                                        elements_disabled.set(false);
                                        if let Some(result) = submit_action.value().get() {
                                            match result {
                                                Ok(_) => "Request sent successfully.".to_owned(),
                                                Err(e) => format!("Failed to send request: {:?}", e)
                                            }
                                        } else {
                                            "".to_owned()
                                        }
                                    }
                                });

                                view! {
                                    <Row>
                                        <div class="flex flex-col flex-1"/>
                                        <Panel>
                                            <Row>
                                                <h1 class="text-3xl font-bold">
                                                    "Region 15 Scholarship Application (DEMO)"
                                                </h1>
                                            </Row>
                                            <Row>
                                                <p class="text-sm">
                                                    "This is a very simple demonstration of the scholarship application written in Rust."
                                                </p>
                                            </Row>
                                            <Row>
                                                // Each OutlinedTextField could be simplified later into an enum:
                                                // Text { data_member, label, placeholder }, i.e.
                                                // Text { "first_name", "First Name:", "John" }
                                                <OutlinedTextField
                                                    label="First Name:"
                                                    placeholder="John"
                                                    disabled=elements_disabled
                                                    data_member = "first_name"
                                                    data_map = expandable_react.data
                                                />
                                                <OutlinedTextField
                                                    label="Last Name:"
                                                    placeholder="Smith"
                                                    disabled=elements_disabled
                                                    data_member = "last_name"
                                                    data_map = expandable_react.data
                                                />
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="Contact Email:"
                                                    placeholder="student@region15.org"
                                                    disabled=elements_disabled
                                                    data_member = "contact_email"
                                                    data_map = expandable_react.data
                                                />
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="Phone Number:"
                                                    placeholder="123-456-7890"
                                                    disabled=elements_disabled
                                                    data_member = "phone_number"
                                                    data_map = expandable_react.data
                                                />
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="Street Address:"
                                                    placeholder="123 Fake Street"
                                                    disabled=elements_disabled
                                                    data_member = "address"
                                                    data_map = expandable_react.data
                                                />
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="Highest Math SAT Score:"
                                                    placeholder="600"
                                                    disabled=elements_disabled
                                                    data_member = "math_sat"
                                                    data_map = expandable_react.data
                                                    input_type = "number"
                                                />
                                            </Row>
                                            <Row>
                                                // Each RadioList could be simplified into an enum:
                                                // Radio { data_member, label, items }
                                                <RadioList
                                                    label="Town:"
                                                    items=vec!["Southbury", "Middlebury"]
                                                        .into_iter()
                                                        .map(|s| s.into())
                                                        .collect()
                                                    disabled=elements_disabled
                                                    data_member="town"
                                                    data_map = expandable_react.data
                                                />
                                            </Row>
                                            <Row>
                                                // Select { data_member, label, items }
                                                <Select
                                                    label="Gender:"
                                                    value_list=vec!["Male", "Female", "Prefer not to answer"]
                                                        .into_iter()
                                                        .map(|s| s.into())
                                                        .collect()
                                                    disabled=elements_disabled
                                                    data_member="gender"
                                                    data_map=expandable_react.data
                                                />
                                            </Row>
                                            <Row>
                                                <ActionButton
                                                    on:click=move |_| {
                                                        let captured = reactive_info.capture();
                                                        let captured_map = expandable_react.capture();
                                                        console_log(format!("Found values: {captured:?}").as_str());
                                                        console_log(format!("Map values: {captured_map:?}").as_str());
                                                        log_action
                                                            .dispatch(LogExpandableInfo {
                                                                info: captured_map
                                                            });
                                                        // submit_action
                                                        //     .dispatch(CreateSampleSubmission {
                                                        //         student_info: captured,
                                                        //     });
                                                    }
                                                    disabled=elements_disabled
                                                >"Submit"</ActionButton>
                                            </Row>
                                            <Row>
                                                <p>
                                                    {result_msg}
                                                </p>
                                            </Row>
                                        </Panel>
                                        <div class="flex flex-col flex-1"/>
                                    </Row>
                                }
                            })
                    }}
                </Suspense>
            </Authenticated>
        </AuthLoaded>
    }
}
