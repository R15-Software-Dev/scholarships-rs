// Server dependencies
#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{Client, error::ProvideErrorMetadata, types::AttributeValue};
#[cfg(feature = "ssr")]
use serde_dynamo::{from_item, to_item};
#[cfg(feature = "ssr")]
use std::process::{Command, Stdio};
#[cfg(feature = "ssr")]
use std::io::Write;

use crate::common::ExpandableInfo;
use crate::pages::UnauthenticatedPage;
use crate::pages::utils::get_user_claims;
use crate::components::{ActionButton, CheckboxList, Loading, MultiEntry, OutlinedTextField, Panel, RadioList, Row, Select, TextFieldType, ValidatedForm};
use crate::input;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use base64::Engine;
use leptos::logging::log;
use leptos::task::spawn_local;
use leptos::web_sys::HtmlAnchorElement;
use leptos::wasm_bindgen::JsCast;
use std::collections::HashMap;

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

#[server(CreateExamplePdf, endpoint = "/example-pdf")]
pub async fn create_example_pdf() -> Result<Vec<u8>, ServerFnError> {
    // Create typst template. This will be replaced with getting the template from S3 in the future.
    // The path I'm thinking, for now, is that we generate a series of "members" from the available
    // keys that a student contains, prefixed with "student_". This means that to get the member
    // "unweighted_gpa" we'll use the preprocess variable "student_unweighted_gpa".
    let template = r#"
    = Testing Typst
    This form will create a new student application. The PDF that's generated (like this one)
    is a template that will be used later. We want to preprocess these templates to insert the
    correct values from the API later on.

    = Second Header
    This area is going to do some math because I think that's fun. We'll typeset a simple sum:
    $ sum_(i=0)^(10)
        (n_i) $

    = Third Test
    This is just going to create a bulleted list inside a numbered list:
     + Numbers
     + Number again
       - Not a number.
    "#;

    // Create and run the typst command.
    let mut command = Command::new("typst");
    command
        .arg("compile")
        .arg("-") // Use stdin as input
        .arg("-") // Write output to stdout
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());

    let mut process = command.spawn()?;

    process.stdin.take().unwrap().write(template.as_bytes())?;

    let output = process.wait_with_output()?;
    let pdf_bytes = output.stdout;

    Ok(pdf_bytes)
}

/// The main home page component. Contains a simple contact form.
#[component]
pub fn HomePage() -> impl IntoView {
    //#region Server Resources
    let user_claims = get_user_claims();
    let user_subject = Memo::new(move |_| {
        user_claims.get()
            .map(|claim| claim.claims.subject.clone())
    });

    let form_data = RwSignal::new(HashMap::new());

    let server_resource = Resource::new(
        move || user_subject.get(),
        async move |subject| get_submission(subject.unwrap_or_default()).await
    );
    let submit_action = ServerAction::<CreateSampleSubmission>::new();
    
    Effect::new(move || {
        match server_resource.get() {
            Some(Ok(submission)) => {
                form_data.set(submission.data);
            }
            Some(Err(err)) => {
                log!("Error while getting information: {:?}", err);
            }
            _ => {
                log!("An unknown error occurred.");
            }
        }
    });
    //#endregion
    //#region Component State
    let elements_disabled = Signal::derive(move || {
        submit_action.pending().get()
    });
    //#endregion
    //#region Event logic
    let on_submit = move |_| {
        let mut exp_info = ExpandableInfo::new(user_subject.get().unwrap_or_default());
        exp_info.data = form_data.get();
        submit_action.dispatch(CreateSampleSubmission {
            student_info: exp_info
        });
    };

    let pdf_button_click = move |_| {
        console_log("Attempting to get PDF from server endpoint");
        spawn_local(async move {
            let result = create_example_pdf().await;
            if let Ok(bytes) = result {
                let base64 = base64::engine::general_purpose::STANDARD
                    .encode(bytes);
                let data_url = format!(
                    "data:application/pdf;base64,{}",
                    base64,
                );
                console_log("Found document, opening in new tab...");
                let link = document()
                    .create_element("a")
                    .unwrap()
                    .dyn_into::<HtmlAnchorElement>()
                    .unwrap();
                link.set_href(&*data_url);
                link.set_target("_blank");
                link.click();
            }
        });
    };
    //#endregion

    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                // Replace this fallback with a real loading screen.
                <Suspense fallback=Loading>
                    {move || {
                        server_resource
                            .get()
                            .map(|submission| {
                                view! {
                                    <Row>
                                        <div class="flex flex-col flex-1" />
                                        <Panel>
                                            <ValidatedForm on_submit=Callback::new(on_submit)>
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
                                                    <OutlinedTextField
                                                        label="First Name:"
                                                        placeholder="John"
                                                        disabled=elements_disabled
                                                        data_member="first_name"
                                                        data_map=form_data
                                                    />
                                                    <OutlinedTextField
                                                        label="Last Name:"
                                                        placeholder="Smith"
                                                        disabled=elements_disabled
                                                        data_member="last_name"
                                                        data_map=form_data
                                                    />
                                                </Row>
                                                <Row>
                                                    <OutlinedTextField
                                                        label="Contact Email:"
                                                        placeholder="student@region15.org"
                                                        disabled=elements_disabled
                                                        data_member="contact_email"
                                                        data_map=form_data
                                                        input_type=TextFieldType::Email
                                                    />
                                                </Row>
                                                <Row>
                                                    <OutlinedTextField
                                                        label="Phone Number:"
                                                        placeholder="123-456-7890"
                                                        disabled=elements_disabled
                                                        data_member="phone_number"
                                                        data_map=form_data
                                                    />
                                                </Row>
                                                <Row>
                                                    <OutlinedTextField
                                                        label="Street Address:"
                                                        placeholder="123 Fake Street"
                                                        disabled=elements_disabled
                                                        data_member="address"
                                                        data_map=form_data
                                                    />
                                                </Row>
                                                <Row>
                                                    <OutlinedTextField
                                                        label="Highest Math SAT Score:"
                                                        placeholder="600"
                                                        disabled=elements_disabled
                                                        data_member="math_sat"
                                                        data_map=form_data
                                                        input_type=TextFieldType::Number
                                                    />
                                                </Row>
                                                <Row>
                                                    <RadioList
                                                        label="Town:"
                                                        items=vec!["Southbury", "Middlebury"]
                                                            .into_iter()
                                                            .map(|s| s.into())
                                                            .collect()
                                                        disabled=elements_disabled
                                                        data_member="town"
                                                        data_map = form_data
                                                    />
                                                </Row>
                                                <Row>
                                                    <CheckboxList
                                                        label="Favorite Candies:"
                                                        items=vec!["Twizzlers", "Reese's", "Starburst"]
                                                            .into_iter()
                                                            .map(|s| s.into())
                                                            .collect()
                                                        disabled=elements_disabled
                                                        data_member="favorite_candies"
                                                        data_map=form_data
                                                        required=true
                                                    />
                                                </Row>
                                                <Row>
                                                    <Select
                                                        label="Gender:"
                                                        value_list=vec!["Male", "Female", "Prefer not to answer"]
                                                            .into_iter()
                                                            .map(|s| s.to_owned())
                                                            .collect()
                                                        disabled=elements_disabled
                                                        data_member="gender"
                                                        data_map=form_data
                                                    />
                                                </Row>
                                                <Row>
                                                    <MultiEntry
                                                        data_map=form_data
                                                        data_member="community_involvement"
                                                        schema=vec![
                                                            input!(Text, "service_name", "Activity Name:", "Some service activity..."),
                                                            input!(Number, "service_hours", "Total Service Hours", "20")
                                                        ]
                                                    />
                                                </Row>
                                            </ValidatedForm>
                                        </Panel>
                                        <div class="flex flex-col flex-1" />
                                    </Row>
                                }
                            })
                    }}
                    <Row>
                        <ActionButton on:click=pdf_button_click>"Get PDF"</ActionButton>
                    </Row>
                </Suspense>
            </Authenticated>
        </AuthLoaded>
    }
}
