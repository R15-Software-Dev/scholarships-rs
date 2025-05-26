use crate::components::{ActionButton, OutlinedTextField};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{error::ProvideErrorMetadata, types::AttributeValue, Client};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentInfo {
    first_name: String,
    last_name: String,
}

#[server(GetSubmission, endpoint = "/get-submission")]
pub async fn get_submission(id: String) -> Result<StudentInfo, ServerFnError> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    console_log(format!("Getting values from API using key {}", id).as_str());

    match dbclient
        .get_item()
        .table_name("student-applications")
        .key("Email", AttributeValue::S(id))
        .send()
        .await
    {
        Ok(output) => {
            console_log("Found submission with passed key.");
            let item = output.item.unwrap();
            let first_name = item
                .get("studentFirstName")
                .unwrap()
                .as_s()
                .unwrap()
                .clone();
            let last_name = item.get("studentLastName").unwrap().as_s().unwrap().clone();
            Ok(StudentInfo {
                first_name,
                last_name,
            })
        }
        Err(err) => {
            let msg = err.message().unwrap_or("Unknown error");
            // console_log(format!("Exception while getting submission information: {}", msg).as_str());
            Err(ServerFnError::new(msg))
        }
    }
}

#[server(CreateSampleSubmission, endpoint = "/create-sample-submission")]
pub async fn create_sample_submission(student_info: StudentInfo) -> Result<(), ServerFnError> {
    use aws_sdk_dynamodb::{types::AttributeValue, Client};

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    console_log(
        format!(
            "Creating sample submission with name {} {}",
            student_info.first_name, student_info.last_name
        )
        .as_str(),
    );

    let submission_id = String::from("testing_student");

    match dbclient
        .update_item()
        .table_name("student-applications")
        .key("Email", AttributeValue::S(submission_id))
        .expression_attribute_values(
            ":studentFirstName",
            AttributeValue::S(student_info.first_name),
        )
        .expression_attribute_values(
            ":studentLastName",
            AttributeValue::S(student_info.last_name),
        )
        .expression_attribute_names("#studentFirstName", "studentFirstName")
        .expression_attribute_names("#studentLastName", "studentLastName")
        .update_expression(
            "SET #studentFirstName = :studentFirstName, #studentLastName = :studentLastName",
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
    let sample_id = RwSignal::new("google_113247439743075864879".to_string());
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let elements_disabled = RwSignal::new(false);

    let server_resource = Resource::new(
        move || sample_id.get(),
        async |id| {
            get_submission(id).await.unwrap_or_else(|e| {
                console_log(e.to_string().as_str());
                StudentInfo {
                    first_name: String::from("Error"),
                    last_name: String::from(""),
                }
            })
        },
    );
    let on_click = move |_| {
        *count.write() += 1;
        *elements_disabled.write() = count.get() == 10;
    };
    let submit_action = ServerAction::<CreateSampleSubmission>::new();

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <p>
            "Lillian's reported full name from the API: "
            <Suspense fallback=move || view! { <span>"Loading..."</span> }>
                {move || {
                    server_resource.get().map(|submission| {
                        view! { <span>{submission.first_name}" "{submission.last_name}</span> }
                    })
                }}
            </Suspense>
        </p>
        <ActionForm action=submit_action>
            <div>
                <OutlinedTextField
                    label="First Name".into()
                    placeholder="John".into()
                    disabled={elements_disabled}
                    name="student_info[first_name]".into() />
            </div>
            <div>
                <OutlinedTextField
                    label="Last Name".into()
                    placeholder="Smith".into()
                    disabled={elements_disabled}
                    name="student_info[last_name]".into() />
            </div>
            <div>
                <ActionButton
                    on:click=on_click
                    disabled={elements_disabled}
                    button_type="submit".to_string()
                >
                    "Submit"
                </ActionButton>
            </div>
        </ActionForm>
    }
}
