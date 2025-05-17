use leptos::prelude::*;
use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use leptos::server_fn::serde::{Serialize, Deserialize};
use crate::components::{ActionButton, OutlinedTextField};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentSubmission {
    testing_value: Option<String>
}

#[server]
pub async fn create_sample_submission() -> Result<StudentSubmission, ServerFnError> {
    let config = aws_config::load_defaults().await;
    let dbclient = aws_sdk_dynamodb::Client::new(&config);

    match dbclient
        .get_item()
        .table_name("student-applications")
        .key("Email", dynamodb::model::AttributeValue::S("google_1234567".to_string()))
        .send()
        .await
    {
        Ok(output) => {
            println!("Found submission with passed key.");
            Ok(StudentSubmission {
                testing_value: None
            })
        }
        Err(e) => Err(Error::unhandled(e))
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
    };
    let get_submission = ServerAction::<CreateSampleSubmission>::new();

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <OutlinedTextField 
            placeholder="Testing...".to_string()
            disabled={elements_disabled} />
        <ActionButton on:click=on_click disabled={elements_disabled}>"Click Me: " {count}</ActionButton>
    }
}
