use crate::components::{ActionButton, OutlinedTextField};
use leptos::prelude::*;
use leptos::leptos_dom::logging::console_log;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentSubmission {
    first_name: String,
    last_name: String
}

#[server(CreateSampleSubmission)]
pub async fn create_sample_submission() -> Result<StudentSubmission, ServerFnError> {
    use aws_sdk_dynamodb::{types::AttributeValue, Client};

    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let dbclient = Client::new(&config);

    console_log("Getting values from API");

    match dbclient
        .get_item()
        .table_name("student-applications")
        .key("Email", AttributeValue::S("google_113247439743075864879".to_string()))
        .send()
        .await
    {
        Ok(output) => {
            println!("Found submission with passed key.");
            let item = output.item.unwrap();
            let first_name = item.get("studentFirstName").unwrap().as_s().unwrap().clone();
            let last_name = item.get("studentLastName").unwrap().as_s().unwrap().clone();
            Ok(StudentSubmission {
                first_name: first_name,
                last_name: last_name
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
    let server_resource = Resource::new(|| {}, |_| async {
        create_sample_submission().await.unwrap()
    });
    let on_click = move |_| {
        *count.write() += 1;
        *elements_disabled.write() = count.get() == 10;
    };

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
        <OutlinedTextField
            placeholder="Testing...".to_string()
            disabled={elements_disabled} />
        <ActionButton on:click=on_click disabled={elements_disabled}>"Click Me: " {count}</ActionButton>
    }
}
