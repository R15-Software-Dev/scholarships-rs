use leptos::prelude::*;
use std::collections::HashMap;
use crate::common::ValueType;
use leptos::logging::log;

#[cfg(feature = "ssr")]
mod server_logic {
    pub use crate::utils::server::create_dynamo_client;
    pub use super::super::PROVIDER_CONTACT_TABLE;
}

#[server]
pub async fn get_all_providers() -> Result<Vec<HashMap<String, ValueType>>, ServerFnError> {
    use server_logic::*;

    // We want to get all provider information from the contacts table.
    // This will be migrated to the new table structure when it happens.

    let client = create_dynamo_client().await;

    log!("Scanning all provider information from {}", PROVIDER_CONTACT_TABLE);

    client
        .scan()
        .table_name(PROVIDER_CONTACT_TABLE)
        .send()
        .await
        .map(|output| {
            if let Some(items) = output.items {
                items
                    .into_iter()
                    .map(|item| {
                        item.into_iter().map(|(key, attr)| {
                            let val = ValueType::from(&attr);
                            (key, val)
                        }).collect()
                    })
                    .collect::<Vec<HashMap<String, ValueType>>>()
            } else {
                Vec::new()
            }
        })
        .map_err(ServerFnError::from)
}
