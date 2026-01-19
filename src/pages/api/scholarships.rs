#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{
    types::AttributeValue,
    error::ProvideErrorMetadata,
};
#[cfg(feature = "ssr")]
use leptos::leptos_dom::log;
#[cfg(feature = "ssr")]
use uuid::Uuid;
#[cfg(feature = "ssr")]
use crate::utils::server::{create_dynamo_client, into_attr_map};
#[cfg(feature = "ssr")]
use crate::common::ValueType;

use crate::common::ExpandableInfo;

use leptos::prelude::ServerFnError;
use leptos::server;

#[cfg(feature = "ssr")]
static SCHOLARSHIPS_TABLE: &str = "leptos-scholarships";

#[server(GetScholarshipInfo)]
pub async fn get_scholarship_info(id: String) -> Result<ExpandableInfo, ServerFnError> {
    let client = create_dynamo_client().await;

    // Perform the operation - we just want to return all data that's contained in this entry,
    // or just return an empty ExpandableInfo struct.
    log!("Getting scholarship info using id {:?}", id);
    match client
        .get_item()
        .table_name(SCHOLARSHIPS_TABLE)
        .key("subject", AttributeValue::S(id.clone()))
        .send()
        .await
    {
        Ok(output) => {
            if let Some(item) = output.item {
                log!("Found output from API: {:?}", item);
                Ok(serde_dynamo::from_item(item)?)
            } else {
                log!("Couldn't find scholarship with ID {:?}", id);
                Err(ServerFnError::new("Couldn't find scholarship with given ID."))
            }
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server(CreateScholarshipInfo)]
pub async fn create_scholarship_info(info: ExpandableInfo) -> Result<(), ServerFnError> {
    use aws_sdk_dynamodb::error::ProvideErrorMetadata;

    let client = create_dynamo_client().await;

    log!("Creating or updating scholarship with ID {:?}", info.subject);

    match client
        .put_item()
        .table_name(SCHOLARSHIPS_TABLE)
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

#[server(GetAllScholarshipInfo)]
pub async fn get_all_scholarship_info() -> Result<Vec<ExpandableInfo>, ServerFnError> {
    let client = create_dynamo_client().await;
    log!("Getting all scholarship info");
    match client
        .scan()
        .table_name(SCHOLARSHIPS_TABLE)
        .send()
        .await
    {
        Ok(output) => {
            if let Some(items) = output.items {
                Ok(serde_dynamo::from_items(items)?)
            } else {
                Ok(Vec::new())
            }
        },
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

/// Gets all scholarships that are associated with the given scholarship provider's ID.
#[server(GetProviderScholarships)]
pub async fn get_provider_scholarships(provider_id: String) -> Result<Vec<ExpandableInfo>, ServerFnError> {
    let client = create_dynamo_client().await;
    
    log!("Getting provider scholarships for provider with ID {:?}", provider_id);
    
    match client
        .scan()
        .table_name(SCHOLARSHIPS_TABLE)
        .expression_attribute_values(":id", serde_dynamo::to_attribute_value(ValueType::String(Some(provider_id)))?)
        .filter_expression("provider_id = :id")
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

/// Creates a new scholarship with a unique ID, and then returns that ID.
#[server(RegisterScholarship)]
pub async fn register_scholarship(provider_id: String) -> Result<String, ServerFnError> {
    let client = create_dynamo_client().await;
    
    log!("Creating scholarship for provider with ID {:?}", provider_id);
    
    let mut current_uuid = Uuid::new_v4().to_string();
    let mut item = ExpandableInfo::new(current_uuid.clone());
    item.data.insert("provider_id".to_string(), ValueType::String(Some(provider_id)));
    loop {
        let ser_item = serde_dynamo::to_item(&item)?;
        match client
            .put_item()
            .table_name(SCHOLARSHIPS_TABLE)
            .set_item(Some(ser_item))
            .condition_expression("attribute_not_exists(subject)")
            .send()
            .await
        {
            Ok(_) => {
                // Return the uuid that we used.
                return Ok(current_uuid);
            }
            Err(err) => {
                // Check the error to figure out if it was a key validation error.
                // If so, retry with new values.
                match err.code() {
                    Some("ConditionalCheckFailedException") => {
                        log!("Retrying conditional check.");
                        log!("Current error: {:?}", err);
                        current_uuid = Uuid::new_v4().to_string();
                        item.subject = current_uuid.clone();
                        continue
                    }
                    _ => {
                        return Err(ServerFnError::new(err.message().unwrap_or("An unknown error occurred")));
                    }
                }
            }
        }
    }
}

/// Deletes a provider's scholarship given their provider ID and scholarship ID.
#[server(DeleteProviderScholarship)]
pub async fn delete_provider_scholarship(provider_id: String, scholarship_id: String) -> Result<(), ServerFnError> {
    let client = create_dynamo_client().await;
    
    log!("Deleting scholarship with ID {:?} for provider with ID {:?}", scholarship_id, provider_id);
    
    // When we delete a scholarship, we need to ensure that the provider's ID matches the scholarship,
    // otherwise everyone can delete anyone else's scholarships.
    match client
        .delete_item()
        .table_name(SCHOLARSHIPS_TABLE)
        .key("subject", AttributeValue::S(scholarship_id))
        .expression_attribute_values(":provider_id", serde_dynamo::to_attribute_value(ValueType::String(Some(provider_id)))?)
        .condition_expression("provider_id = :provider_id")
        .send()
        .await
    {
        Ok(_) => {
            Ok(())
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}
