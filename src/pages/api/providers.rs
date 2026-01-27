use leptos::prelude::*;
use leptos::logging::log;
use std::collections::HashMap;
use crate::common::ValueType;

#[cfg(feature = "ssr")]
use crate::utils::server::{create_dynamo_client, into_attr_map};

#[cfg(feature = "ssr")]
static PROVIDER_CONTACT_TABLE: &str = "leptos-provider-contacts";

#[server]
pub async fn get_provider_contact(id: String) -> Result<HashMap<String, ValueType>, ServerFnError> {
    use aws_sdk_dynamodb::types::AttributeValue;
    use aws_sdk_dynamodb::error::ProvideErrorMetadata;

    let client = create_dynamo_client().await;

    log!("Getting contact info for provider {}", id);

    match client
        .get_item()
        .table_name(PROVIDER_CONTACT_TABLE)
        .key("subject", AttributeValue::S(id))
        .send()
        .await
    {
        Ok(output) => {
            let Some(item) = output.item else {
                return Ok(HashMap::new());
            };

            let map = item.iter().map(|(key, val)| {
                let val_type = ValueType::from(val);

                (key.clone(), val_type)
            }).collect::<HashMap<String, ValueType>>();

            Ok(map)
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred.");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server]
pub async fn put_provider_contact(id: String, contact_info: HashMap<String, ValueType>) -> Result<(), ServerFnError> {
    use aws_sdk_dynamodb::error::ProvideErrorMetadata;

    let client = create_dynamo_client().await;

    let mut item = into_attr_map(contact_info);
    item.insert("subject".to_owned(), aws_sdk_dynamodb::types::AttributeValue::S(id));

    match client
        .put_item()
        .table_name(PROVIDER_CONTACT_TABLE)
        .set_item(Some(item))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred.");
            Err(ServerFnError::new(msg))
        }
    }
}