use leptos::prelude::*;

#[server]
pub async fn put_student_data(
    subject: String,
    data_type: String,
    data_map: std::collections::HashMap<String, crate::common::ValueType>
) -> Result<(), ServerFnError> {
    use leptos::logging::log;
    use crate::utils::server::{create_dynamo_client, into_attr_map};
    use aws_sdk_dynamodb::types::AttributeValue;
    use super::MAIN_TABLE_NAME;
    
    let client = create_dynamo_client().await;
    
    let mut data_map_attr = into_attr_map(data_map);
    data_map_attr.insert(
        "HK".into(),
        AttributeValue::S(format!("STUDENT#{}", subject))
    );
    data_map_attr.insert(
        "SK".into(),
        AttributeValue::S(data_type.to_uppercase())
    );
    
    log!("Inserting this item: {:?}", data_map_attr);
    
    client
        .put_item()
        .table_name(MAIN_TABLE_NAME)
        .set_item(Some(data_map_attr))
        .send()
        .await
        .map(|_| ())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn get_student_data(
    subject: String,
    data_type: String,
) -> Result<std::collections::HashMap<String, crate::common::ValueType>, ServerFnError> {
    use crate::utils::server::create_dynamo_client;
    use std::collections::HashMap;
    use crate::common::ValueType;
    use aws_sdk_dynamodb::types::AttributeValue;
    use super::MAIN_TABLE_NAME;
    
    let client = create_dynamo_client().await;
    
    client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .key_condition_expression("HK = :hk AND SK = :sk")
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{}", subject)))
        .expression_attribute_values(":sk", AttributeValue::S(data_type.to_uppercase()))
        .send()
        .await
        .map(|output| {
            // Map the output to a HashMap<String, ValueType>
            let Some(list) = output.items else {
                return HashMap::new();
            };
            
            let mut map = HashMap::new();
            
            let _ = list
                .into_iter()
                .flatten()
                .for_each(|(k, v)| {
                    map.insert(k, ValueType::from(&v));
                });
            
            map
        })
        .map_err(ServerFnError::from)
}
