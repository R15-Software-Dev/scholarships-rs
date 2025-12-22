#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{error::ProvideErrorMetadata, types::AttributeValue};

use crate::common::ExpandableInfo;
use leptos::leptos_dom::log;
use leptos::prelude::ServerFnError;
use leptos::server;

#[cfg(feature = "ssr")]
use crate::pages::utils::server_utils::create_dynamo_client;

#[server(GetScholarshipInfo, endpoint = "/scholarship/info/get")]
pub async fn get_scholarship_info(id: String) -> Result<ExpandableInfo, ServerFnError> {
    let client = create_dynamo_client().await;

    // Perform the operation - we just want to return all data that's contained in this entry,
    // or just return an empty ExpandableInfo struct.
    log!("Getting scholarship info using id {:?}", id);
    match client
        .get_item()
        .table_name("leptos-scholarship-test")
        .key("subject", AttributeValue::S(id.clone()))
        .send()
        .await
    {
        Ok(output) => {
            if let Some(item) = output.item {
                log!("Found output from API: {:?}", item);
                Ok(serde_dynamo::from_item(item)?)
            } else {
                log!("Couldn't find any values, returning default struct.");
                Ok(ExpandableInfo::new(id.clone()))
            }
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server(CreateScholarshipInfo, endpoint = "/scholarship/info/create")]
pub async fn create_scholarship_info(info: ExpandableInfo) -> Result<(), ServerFnError> {
    use crate::pages::utils::server_utils::create_dynamo_client;
    use aws_sdk_dynamodb::error::ProvideErrorMetadata;

    let client = create_dynamo_client().await;

    log!(
        "Creating or updating scholarship with ID {:?}",
        info.subject
    );

    match client
        .put_item()
        .table_name("leptos-scholarship-test")
        .set_item(Some(serde_dynamo::to_item(&info)?))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server(GetAllScholarshipInfo, endpoint = "/scholarship/info/get-all")]
pub async fn get_all_scholarship_info() -> Result<Vec<ExpandableInfo>, ServerFnError> {
    let client = create_dynamo_client().await;
    log!("Getting all scholarship info");
    match client
        .scan()
        .table_name("leptos-scholarship-test")
        .send()
        .await
    {
        Ok(output) => {
            if let Some(items) = output.items {
                Ok(serde_dynamo::from_items(items)?)
            } else {
                Ok(Vec::new())
            }
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}
