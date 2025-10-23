// Server dependencies
#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{Client, error::ProvideErrorMetadata, types::AttributeValue};

#[cfg(feature = "ssr")]
use serde_dynamo::{from_item, to_item};

use crate::common::{ExpandableInfo, UserClaims};
use crate::pages::UnauthenticatedPage;
use crate::components::{
    ActionButton, CheckboxList, Loading, MultiEntry, OutlinedTextField, Panel, RadioList, Row,
    Select, ChipsList
};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_oidc::{Algorithm, AuthLoaded, AuthSignal, Authenticated};
use std::collections::HashMap;
use traits::{AsReactive, ReactiveCapture};

/// # Get Student Info
/// Gets a student's information given their `subject`.
///
/// All information is found by using a `GetItemCommand` in the student application DynamoDB table.
#[server(GetSubmission, endpoint = "/get-submission")]
pub async fn get_submission(subject: String) -> Result<ExpandableInfo, ServerFnError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    console_log(format!("Getting values from API using subject {}", subject).as_str());

    // Gets the item from the database. It only returns an error if the database
    // experiences an error - not if the item is not found.
    match dbclient
        .get_item()
        .table_name("leptos-test")
        .key("subject", AttributeValue::S(subject.clone()))
        .send()
        .await
    {
        Ok(output) => {
            if let Some(item) = output.item {
                console_log(format!("Found item from API: {item:?}").as_str());
                Ok(from_item(item)?)
            } else {
                console_log("Couldn't find entry, returning default.");
                let info = ExpandableInfo::new(subject);
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
pub async fn create_sample_submission(student_info: ExpandableInfo) -> Result<(), ServerFnError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    // The put_item action can create or update an item in a DynamoDB table.
    // It will completely replace any existing item with the same primary key,
    // which is the desired behavior.
    let item = to_item(student_info.clone())?;
    console_log(format!("Arguments for sample submission: {:?}", item).as_str());
    match dbclient
        .put_item()
        .table_name("leptos-test")
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
    let item: HashMap<String, AttributeValue> = to_item(info)?;
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
    let server_resource: Resource<ExpandableInfo> = Resource::new(
        move || user_claims.get().map(|claim| claim.claims.subject.clone()),
        async |opt_username| match opt_username {
            Some(subject) => get_submission(subject.clone()).await.unwrap_or_else(|e| {
                console_log(e.to_string().as_str());
                ExpandableInfo::new(subject)
            }),
            None => ExpandableInfo::new("".to_owned()),
        },
    );
    let submit_action = ServerAction::<CreateSampleSubmission>::new();
    let log_action = ServerAction::<LogExpandableInfo>::new();

    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                // Replace this fallback with a real loading screen.
                <Suspense fallback=Loading>
                    {move || {
                        server_resource
                            .get()
                            .map(|submission| {
                                let expandable_react = submission.as_reactive();
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
                                                // Text { data_member, label, placeholder }
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
                                                // Checkbox { data_member, label, items }
                                                <CheckboxList
                                                    label="Favorite Candies:"
                                                    items=vec!["Twizzlers", "Reese's", "Starburst"]
                                                        .into_iter()
                                                        .map(|s| s.into())
                                                        .collect()
                                                    disabled=elements_disabled
                                                    data_member="favorite_candies"
                                                    data_map=expandable_react.data
                                                />
                                            </Row>
                                            <Row>
                                                // Select { data_member, label, items }
                                                <Select
                                                    label="Gender:"
                                                    value_list=vec!["Male", "Female", "Prefer not to answer"]
                                                        .into_iter()
                                                        .map(|s| s.to_owned())
                                                        .collect()
                                                    disabled=elements_disabled
                                                    data_member="gender"
                                                    data_map=expandable_react.data
                                                />
                                            </Row>
                                            <Row>
                                                <ChipsList
                                                    data_member="athletic_requirements"
                                                    data_map=expandable_react.data
                                                    items=vec!(
                                                            "Football", "Soccer", "Cross Country", "Cheerleading",
                                                            "Swimming", "Wrestling", "Ski", "Basketball",
                                                            "Lacrosse", "Softball", "Indoor/Outdoor Track",
                                                            "Golf", "Tennis", "Volleyball"
                                                        )
                                                        .into_iter().map(|s| s.to_owned())
                                                        .collect()
                                                    disabled=elements_disabled
                                                    label="Sports/Athletic Requirements"
                                                />
                                            </Row>
                                            <Row>
                                                <ChipsList
                                                    data_member="community_involvement"
                                                    data_map=expandable_react.data
                                                    items=vec!(
                                                            "Lion's Club", "Knights of Columbus",
                                                            "Community Service > 20hrs"
                                                        )
                                                        .into_iter().map(|s| s.to_owned())
                                                        .collect()
                                                    disabled=elements_disabled
                                                    label="Required Community Involvement"
                                            </Row>
                                            <Row>
                                                <MultiEntry
                                                    data_map = expandable_react.data
                                                    data_member = "test"
                                                    name_member = "first_name"
                                                    schema = vec![
                                                        input!(Text, "first_name", "First Name:", "John"),
                                                        input!(Text, "last_name", "Last Name:", "Smith"),
                                                        input!(Select, "gender", "Gender:", ["Male", "Female"]),
                                                        input!(Checkbox, "candy", "Favorite Candy:", ["Twizzlers", "Starburst"])
                                                    ]
                                                />
                                            </Row>
                                            <Row>
                                                <ActionButton
                                                    on:click=move |_| {
                                                        let captured_map = expandable_react.capture();
                                                        console_log(format!("Map values: {:?}", captured_map).as_str());
                                                        submit_action.dispatch(CreateSampleSubmission {
                                                            student_info: captured_map,
                                                        });
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
