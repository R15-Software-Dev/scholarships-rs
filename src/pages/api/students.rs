use leptos::prelude::*;
use leptos::server_fn::codec::Json;
use std::collections::HashMap;

#[cfg(feature = "ssr")]
mod imports {
    pub use super::super::MAIN_TABLE_NAME;
    pub use crate::common::ValueType;
    pub use crate::pages::api::tokens::validate_and_get_token_info;
    pub use crate::utils::server::*;
    pub use aws_sdk_dynamodb::error::ProvideErrorMetadata;
    pub use aws_sdk_dynamodb::types::AttributeValue;
    pub use leptos::logging::{debug_log, error};
    pub use std::collections::HashMap;
}

#[server(input = Json)]
pub async fn put_student_data(
    subject: String,
    data_type: String,
    data_map: HashMap<String, crate::common::ValueType>,
) -> Result<(), ServerFnError> {
    use imports::*;

    let client = create_dynamo_client().await;

    let mut data_map_attr = into_attr_map(data_map);
    data_map_attr.insert(
        "HK".into(),
        AttributeValue::S(format!("STUDENT#{}", subject)),
    );
    data_map_attr.insert("SK".into(), AttributeValue::S(data_type.to_uppercase()));

    debug_log!("Inserting this item: {:?}", data_map_attr);

    client
        .put_item()
        .table_name(MAIN_TABLE_NAME)
        .set_item(Some(data_map_attr))
        .send()
        .await
        .map(|_| ())
        .map_err(|err| {
            let msg = format!(
                "Couldn't put item into Dynamo: {}",
                err.message().unwrap_or("Unknown error occurred")
            );
            error!("{}", msg);
            ServerFnError::new(msg)
        })
}

#[server]
pub async fn get_student_data(
    subject: String,
    data_type: String,
) -> Result<HashMap<String, crate::common::ValueType>, ServerFnError> {
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

            let _ = list.into_iter().flatten().for_each(|(k, v)| {
                map.insert(k, ValueType::from(&v));
            });

            map
        })
        .map_err(|err| {
            let msg = format!(
                "Couldn't get item from Dynamo: {}",
                err.message().unwrap_or("Unknown error occurred")
            );
            error!("{}", msg);
            ServerFnError::new(msg)
        })
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
    subject: String,
) -> Result<HashMap<String, crate::common::ValueType>, ServerFnError> {
    use imports::*;

    let client = create_dynamo_client().await;

    client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .key_condition_expression("HK = :hk")
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{}", subject)))
        .send()
        .await
        .map(|output| {
            let Some(items) = output.items else {
                return HashMap::new();
            };

            items
                .iter()
                .flatten()
                .map(|(k, v)| (k.clone(), ValueType::from(v)))
                .collect::<HashMap<String, ValueType>>()
        })
        .map_err(|err| {
            let msg = format!(
                "Couldn't get data from Dynamo: {}",
                err.message().unwrap_or("Unknown error occurred")
            );
            error!("{}", msg);
            ServerFnError::new(msg)
        })
}

#[server]
pub async fn get_completed_students(
    access_token: String,
) -> Result<HashMap<String, HashMap<String, crate::common::ValueType>>, ServerFnError> {
    use imports::*;

    // We want to get all student information. The requirements are that the students have completed
    // the demographics form - everything else may bed from this.
    // The easiest way is to get all the information and filter on this side, instead of bookkeeping
    // on the database's side.

    // We want to verify the access token first, and make sure that the user has the correct access.
    let claims = validate_and_get_token_info(access_token).await?;
    if !claims.groups.contains(&"ScholarshipProviders".to_string()) {
        return Err(ServerFnError::new(
            "User is not in the ScholarshipProviders group",
        ));
    }

    let client = create_dynamo_client().await;

    let response = client
        .scan()
        .table_name(MAIN_TABLE_NAME)
        .send()
        .await
        .map_err(|err| {
            let msg = err.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })?;

    let Some(items) = response.items else {
        return Ok(HashMap::new());
    };

    let mut output = HashMap::<String, HashMap<String, ValueType>>::new();

    items.into_iter().for_each(|mut form_info| {
        let student_id_full = form_info
            .get("HK")
            .map(|v| v.as_s().cloned().unwrap_or_default())
            .unwrap_or_default()
            .to_owned();

        let student_id = student_id_full.split("STUDENT#").collect::<String>();

        form_info.remove("HK");
        form_info.remove("SK");

        let form_info_convert = form_info
            .into_iter()
            .map(|(k, v)| (k, ValueType::from(&v)))
            .collect::<HashMap<String, ValueType>>();

        // We want to insert all the remaining information into the output map
        output
            .entry(student_id)
            .and_modify(|v| {
                v.extend(form_info_convert.clone());
            })
            .or_insert(form_info_convert);
    });

    // Don't love this, but it does verify that the student has completed the demographics form
    let output = output
        .into_iter()
        .filter_map(|(k, student_info)| {
            student_info
                .get("first_name")
                .and_then(|_| Some((k, student_info.clone())))
        })
        .collect();

    Ok(output)
}
