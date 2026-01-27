use std::collections::HashMap;
use leptos::logging::log;
use leptos::prelude::*;
use crate::common::DateInfo;

#[cfg(feature = "ssr")]
use super::DATES_TABLE;

#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{
    types::{WriteRequest, PutRequest},
    Client as DynamoClient
};

#[cfg(feature = "ssr")]
use crate::utils::server::create_dynamo_client;

#[cfg(feature = "ssr")]
use std::sync::Arc;

#[server]
pub async fn create_dates(dates: Vec<DateInfo>) -> Result<(), ServerFnError> {
    // Serialize and put dates into the database.
    let client = create_dynamo_client().await;
    
    log!("Creating dates: {:?}", dates);
    
    let requests = dates
        .into_iter()
        .map(|date| {
            let item = serde_dynamo::to_item(&date)
                .unwrap_or_default();
            
            let put_request = PutRequest::builder()
                .set_item(Some(item))
                .build()
                .expect("Couldn't build request, item is not present");
            
            WriteRequest::builder().put_request(put_request).build()
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
        .map_err(ServerFnError::from)
}

#[server]
pub async fn get_important_dates() -> Result<Vec<DateInfo>, ServerFnError> {
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
                    .filter_map(|item| {
                        serde_dynamo::from_item::<_, DateInfo>(item)
                            .ok()
                    })
                    .collect::<Vec<DateInfo>>()
            } else {
                Vec::new()
            }
        })
        .map_err(ServerFnError::from)
}
