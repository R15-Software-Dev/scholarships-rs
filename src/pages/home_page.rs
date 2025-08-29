// Server dependencies
#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{error::ProvideErrorMetadata, types::AttributeValue, Client};

#[cfg(feature = "ssr")]
use serde_dynamo::{from_item, to_item};

use crate::app::Unauthenticated;
use crate::common::{StudentInfo, StudentInfoReactive, UserClaims};
use crate::components::{
    ActionButton, CheckboxList, Loading, OutlinedTextField, Panel, Row, Select,
};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_oidc::{Algorithm, AuthLoaded, AuthSignal, Authenticated};

#[server(GetSubmission, endpoint = "/get-submission")]
pub async fn get_submission(id: String) -> Result<StudentInfo, ServerFnError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    console_log(format!("Getting values from API using username {}", id).as_str());

    match dbclient
        .get_item()
        .table_name("student-applications")
        .key("Email", AttributeValue::S(id.clone()))
        .send()
        .await
    {
        Ok(output) => {
            let item = output.item.unwrap();

            let info: StudentInfo = from_item(item).unwrap();

            console_log(format!("Got info from API: {:?}", info).as_str());

            Ok(info)
        }
        Err(err) => {
            let msg = err.message().unwrap_or("Unknown error");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server(CreateSampleSubmission, endpoint = "/create-sample-submission")]
pub async fn create_sample_submission(
    student_info: StudentInfo,
    subject: String,
) -> Result<(), ServerFnError> {
    use aws_sdk_dynamodb::Client;

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    // The put_item action can create or update an item in a DynamoDB table.
    // It will completely replace any existing item with the same primary key,
    // which is the desired behavior.
    let item = to_item(student_info).unwrap();
    match dbclient
        .put_item()
        .set_item(Some(item))
        .send().await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(ServerFnError::new(err.into_service_error().to_string())),
    }
}

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
    let server_resource = Resource::new(
        move || user_claims.get().map(|claim| claim.claims.subject.clone()),
        async |opt_username| match opt_username {
            Some(username) => get_submission(username)
                .await
                .unwrap_or_else(|e| {
                    console_log(e.to_string().as_str());
                    StudentInfo::default()
                }
            ),
            None => StudentInfo::default()
        },
    );
    let submit_action = ServerAction::<CreateSampleSubmission>::new();

    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=Unauthenticated>
                // Replace this fallback with a real loading screen.
                <Suspense fallback=Loading>
                    {move || {
                        server_resource
                            .get()
                            .map(|submission: StudentInfo| {
                                let reactive_info = StudentInfoReactive::new(submission);
                                let select_value = RwSignal::new(String::from("Math"));
                                let chk_select = RwSignal::new(vec!["Testing 2".into()]);
                                let elements_disabled = RwSignal::new(false);

                                view! {
                                    <div class="flex flex-row">
                                        <Panel>
                                            <Row>
                                                <p>
                                                    "Testing paragraph! This panel should be the same size as the other."
                                                </p>
                                            </Row>
                                            <Row>
                                                <p>"Current user's subject ID: "{reactive_info.Email}</p>
                                            </Row>
                                        </Panel>
                                        <Panel>
                                            <Row>
                                                <p>
                                                    "Current user's reported full name from the API: "
                                                    {reactive_info.studentFirstName}" "
                                                    {reactive_info.studentLastName}
                                                </p>
                                            </Row>
                                            <Row>
                                                <OutlinedTextField
                                                    label="First Name:"
                                                    placeholder="John"
                                                    disabled=elements_disabled
                                                    value=reactive_info.studentFirstName
                                                />
                                                <OutlinedTextField
                                                    label="Last Name:"
                                                    placeholder="Smith"
                                                    disabled=elements_disabled
                                                    value=reactive_info.studentLastName
                                                />
                                                <OutlinedTextField
                                                    label="Contact Email"
                                                    placeholder="temp@region15.org"
                                                    disabled=elements_disabled
                                                    value=reactive_info.studentEmail
                                                />
                                            </Row>
                                            <Row>
                                                <Select
                                                    value_list=vec!["Math", "English", "Science"]
                                                        .into_iter()
                                                        .map(|s| s.into())
                                                        .collect()
                                                    value=select_value
                                                    disabled=elements_disabled
                                                />
                                                <CheckboxList
                                                    selected=chk_select
                                                    items=vec!["Testing 1", "Testing 2"]
                                                        .into_iter()
                                                        .map(|s| s.into())
                                                        .collect()
                                                    disabled=elements_disabled
                                                />
                                            </Row>
                                            <Row>
                                                <ActionButton on:click=move |_| {
                                                    console_log(
                                                        format!("Found value {:?}", chk_select.get()).as_str(),
                                                    );
                                                    submit_action
                                                        .dispatch(CreateSampleSubmission {
                                                            student_info: reactive_info.capture(),
                                                            subject: user_claims.get().unwrap().claims.subject.clone(),
                                                        });
                                                }>"Submit"</ActionButton>
                                            </Row>
                                        </Panel>
                                    </div>
                                }
                            })
                    }}
                </Suspense>
            </Authenticated>
        </AuthLoaded>
    }
}
