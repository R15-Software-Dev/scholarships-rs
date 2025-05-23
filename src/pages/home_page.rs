use crate::components::{ActionButton, OutlinedTextField};
use leptos::prelude::*;
use leptos::leptos_dom::logging::console_log;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentSubmission {
    testing_value: Option<String>,
}

#[server]
pub async fn create_sample_submission() -> Result<StudentSubmission, ServerFnError> {
    use aws_sdk_dynamodb::{types::AttributeValue, Client};

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    match dbclient
        .get_item()
        .table_name("student-applications")
        .key("Email", AttributeValue::S("google_113247439743075864879".to_string()))
        .send()
        .await
    {
        Ok(output) => {
            println!("Found submission with passed key.");
            let value = output.item
                .unwrap()
                .get("studentFirstName").unwrap()
                .as_s().unwrap().clone();
            Ok(StudentSubmission {
                testing_value: Some(value),
            })
        }
        Err(e) => Err(ServerFnError::new("Couldn't find a submission with that ID."))
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let elements_disabled = RwSignal::new(false);
    let on_click = move |_| {
        *count.write() += 1;
        *elements_disabled.write() = count.get() == 10;
        spawn_local(async {
            let response = create_sample_submission().await;
            match response {
                Ok(submission) => console_log(submission.testing_value.unwrap_or("Default value.".to_string()).as_str()),
                Err(_) => console_log("Failed to get correct information.")
            };
        });
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <OutlinedTextField
            placeholder="Testing...".to_string()
            disabled={elements_disabled} />
        <ActionButton on:click=on_click disabled={elements_disabled}>"Click Me: " {count}</ActionButton>
    }
}
