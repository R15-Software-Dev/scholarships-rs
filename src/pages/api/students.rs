use leptos::prelude::*;
use leptos::server_fn::codec::Json;

#[cfg(feature = "ssr")]
mod imports {
    pub use crate::utils::server::*;
    pub use aws_sdk_dynamodb::types::AttributeValue;
    pub use super::super::MAIN_TABLE_NAME;
    pub use crate::common::ValueType;
    pub use std::collections::HashMap;
    pub use leptos::logging::debug_log;
}

#[server(input = Json)]
pub async fn put_student_data(
    subject: String,
    data_type: String,
    data_map: std::collections::HashMap<String, crate::common::ValueType>
) -> Result<(), ServerFnError> {
    use imports::*;
    
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
    
    debug_log!("Inserting this item: {:?}", data_map_attr);
    
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
    use imports::*;
    
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

/// Gets and flattens all of a student's information from the database, regardless of their defined
/// sort key.
/// 
/// Note: if sorted data from the `put_student_data` function contains the same fields, this
/// function will have unexpected behavior. For example, if a `first_name` field appears in multiple
/// locations in the database, all for this single student, they will overwrite each other in no
/// specific order.
#[server]
pub async fn get_all_student_data(
    subject: String
) -> Result<std::collections::HashMap<String, crate::common::ValueType>, ServerFnError> {
    use imports::*;
    
    let client = create_dynamo_client().await;
    
    client.query()
        .table_name(MAIN_TABLE_NAME)
        .key_condition_expression("HK = :hk")
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{}", subject)))
        .send()
        .await
        .map(|output| {
            let Some(items) = output.items else {
                return HashMap::new();
            };
            
            items.iter().flatten()
                .map(|(|k, v)| (k.clone(), ValueType::from(v)))
                .collect::<HashMap<String, ValueType>>()
        })
        .map_err(ServerFnError::from)
}
