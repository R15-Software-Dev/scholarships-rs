use crate::common::DateInfo;
use leptos::logging::{error, log};
use leptos::prelude::*;
use std::collections::HashMap;

#[cfg(feature = "ssr")]
use super::DATES_TABLE;

#[cfg(feature = "ssr")]
use crate::utils::server::create_dynamo_client;
#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{
    Client as DynamoClient,
    error::ProvideErrorMetadata,
    types::{PutRequest, WriteRequest},
};

#[server]
pub async fn create_dates(dates: Vec<DateInfo>) -> Result<(), ServerFnError> {
    // Serialize and put dates into the database.
    let client = create_dynamo_client().await;

    log!("Creating dates: {:?}", dates);

    let requests = dates
        .into_iter()
        .filter_map(|date| {
            let item = serde_dynamo::to_item(&date).unwrap_or_default();

            let put_request = PutRequest::builder().set_item(Some(item)).build().ok()?;

            Some(WriteRequest::builder().put_request(put_request).build())
        })
        .collect::<Vec<WriteRequest>>();

    let mut request_map = HashMap::new();
    request_map.insert(DATES_TABLE.to_string(), requests);

    client
        .batch_write_item()
        .set_request_items(Some(request_map))
        .send()
        .await
        .map(|_| ())
        .map_err(|err| {
            let msg = err.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })
}

#[server]
pub async fn get_important_dates() -> Result<Vec<DateInfo>, ServerFnError> {
    log!("Getting important dates list");

    let client = create_dynamo_client().await;

    client
        .scan()
        .table_name(DATES_TABLE)
        .send()
        .await
        .map(|output| {
            if let Some(items) = output.items {
                items
                    .into_iter()
                    .filter_map(|item| serde_dynamo::from_item::<_, DateInfo>(item).ok())
                    .collect::<Vec<DateInfo>>()
            } else {
                Vec::new()
            }
        })
        .map_err(|err| {
            let msg = err.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })
}
