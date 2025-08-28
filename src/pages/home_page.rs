#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{error::ProvideErrorMetadata, types::AttributeValue, Client};
use crate::app::Unauthenticated;
use crate::common::{StudentInfo, StudentInfoReactive, UserClaims};
use crate::components::{ActionButton, Loading, OutlinedTextField, Select, CheckboxList, Panel, Row};
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

            let first_name: String = item
                .get("studentFirstName")
                .and_then(|attr| attr.as_s().ok())
                .map(|s| s.to_owned())
                .unwrap_or_default();

            let last_name: String = item
                .get("studentLastName")
                .and_then(|attr| attr.as_s().ok())
                .map(|s| s.to_owned())
                .unwrap_or_default();

            let math_sat_score = item
                .get("mathSAT")
                .and_then(|attr| attr.as_n().ok())
                .map(|n| n.parse::<i32>().unwrap_or(0))
                .unwrap_or_default();

            let student_email = item
                .get("studentEmail")
                .and_then(|attr| attr.as_s().ok())
                .map(|s| s.to_owned())
                .unwrap_or_default();

            console_log(format!("Got values from API: {} {}", first_name, last_name).as_str());

            Ok(StudentInfo {
                first_name,
                last_name,
                math_sat_score,
                student_email,
            })
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
    use aws_sdk_dynamodb::{types::AttributeValue, Client};

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    console_log(
        format!(
            "Creating sample submission with name {} {}, math score {}, and subject {}",
            student_info.first_name, student_info.last_name, student_info.math_sat_score, subject
        )
        .as_str(),
    );

    match dbclient
        .update_item()
        .table_name("student-applications")
        .key("Email", AttributeValue::S(subject))
        .expression_attribute_values(
            ":studentFirstName",
            AttributeValue::S(student_info.first_name),
        )
        .expression_attribute_values(
            ":studentLastName",
            AttributeValue::S(student_info.last_name),
        )
        .expression_attribute_values(
            ":mathScoreSAT",
            AttributeValue::N(student_info.math_sat_score.to_string()),
        )
        .expression_attribute_names("#studentFirstName", "studentFirstName")
        .expression_attribute_names("#studentLastName", "studentLastName")
        .expression_attribute_names("#mathScoreSAT", "mathScoreSAT")
        .update_expression(
            concat!("SET #studentFirstName = :studentFirstName, ",
                "#studentLastName = :studentLastName, ",
                "#mathScoreSAT = :mathScoreSAT"),
        )
        .send()
        .await
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
            Some(username) => get_submission(username).await.unwrap_or_else(|e| {
                console_log(e.to_string().as_str());
                StudentInfo {
                    first_name: String::from("Error"),
                    last_name: String::new(),
                    math_sat_score: 0,
                    student_email: String::new()
                }
            }),
            None => StudentInfo {
                first_name: String::new(),
                last_name: String::new(),
                math_sat_score: 0,
                student_email: String::new()
            },
        },
    );
    let submit_action = ServerAction::<CreateSampleSubmission>::new();

    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=Unauthenticated>
                // Replace this fallback with a real loading screen.
                <Suspense fallback=Loading>
                    {move || {
                        server_resource.get().map(|submission: StudentInfo| {
                            let reactive_info = StudentInfoReactive::new(submission);

                            let select_value = RwSignal::new(String::from("Math"));
                            let chk_select = RwSignal::new(vec!["Testing 2".into()]);

                            let elements_disabled = RwSignal::new(false);

                            view! {
                                <div class="flex flex-row">
                                <Panel>
                                    <p>"Testing paragraph! This panel should be the same size as the other."</p>
                                </Panel>
                                <Panel>
                                    <Row>
                                        <p>
                                            "Current user's reported full name from the API: "
                                            {reactive_info.first_name}" "{reactive_info.last_name}
                                        </p>
                                    </Row>
                                    <Row>
                                        <OutlinedTextField
                                            label="First Name:"
                                            placeholder="John"
                                            disabled=elements_disabled
                                            value=reactive_info.first_name />
                                        <OutlinedTextField
                                            label="Last Name:"
                                            placeholder="Smith"
                                            disabled=elements_disabled
                                            value=reactive_info.last_name />
                                        <OutlinedTextField
                                            label="Contact Email"
                                            placeholder="temp@region15.org"
                                            disabled=elements_disabled
                                            value=reactive_info.student_email />
                                    </Row>
                                    <Row>
                                        <Select
                                            value_list = vec!["Math".into(), "English".into(), "Science".into()]
                                            value = select_value
                                            disabled=elements_disabled
                                        />
                                        <CheckboxList
                                            selected = chk_select
                                            items = vec!["Testing 1".into(), "Testing 2".into()]
                                            disabled=elements_disabled />
                                    </Row>
                                    <Row>
                                        <ActionButton
                                            on:click=move |_| {
                                                console_log(format!("Found value {:?}", chk_select.get()).as_str());
                                                submit_action.dispatch(CreateSampleSubmission {
                                                    student_info: reactive_info.capture(),
                                                    subject: user_claims.get().unwrap().claims.subject.clone()
                                                });
                                            }
                                        >
                                            "Submit"
                                        </ActionButton>
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
