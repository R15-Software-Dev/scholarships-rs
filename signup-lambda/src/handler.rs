use lambda_runtime::LambdaEvent;
use aws_sdk_cognitoidentityprovider::Client;
use aws_config::BehaviorVersion;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use aws_sdk_cognitoidentityprovider::error::ProvideErrorMetadata;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostConfirmationEvent {
    version: String,
    trigger_source: String,
    region: String,
    user_pool_id: String,
    user_name: String,
    caller_context: CallerContext,
    request: Request,
    response: Response
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CallerContext {
    client_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Request {
    user_attributes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response {}

fn get_error_message(err: impl ProvideErrorMetadata) -> String {
    err.message().unwrap_or("An unknown error occurred").to_string()
}

async fn add_user_to_group(provider: Client, user_id: String, group_name: impl Into<String>) -> Result<(), String> {
    let group_name = group_name.into();
    println!("Adding user {} to group {}", user_id, group_name);
    match provider
        .admin_add_user_to_group()
        .user_pool_id("us-east-1_Lfjuy5zaM")
        .username(user_id)
        .group_name(group_name)
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err(get_error_message(err)),
    }
}

pub(crate) async fn handler(event: LambdaEvent<PostConfirmationEvent>) -> Result<PostConfirmationEvent, String> {
    println!("Found event: {:?}", event.payload);

    let user_id = event.payload.request.user_attributes.get("sub").unwrap().to_string();
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    // Check the client ID of the request to assign the user to the correct location.
    let res = match event.payload.caller_context.client_id.as_str() {
        "10jr2h3vtpu9n7gj46pvg5qo2q" => add_user_to_group(client, user_id, "ScholarshipStudents").await,
        "56c2bqvl021rv8d5mq36blt7jv" => add_user_to_group(client, user_id, "ScholarshipProviders").await,
        _ => Err("Invalid client ID, denying all user access.".to_string()),
    };

    if res.is_ok() {
        Ok(event.payload)
    } else {
        Err(res.unwrap_err())
    }
}
