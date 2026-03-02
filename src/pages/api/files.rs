use leptos::logging::debug_log;
use leptos::prelude::*;
use server_fn::codec::{MultipartData, MultipartFormData};

#[server(input = MultipartFormData)]
pub async fn upload_file(data: MultipartData) -> Result<String, ServerFnError> {
    use super::{MAIN_TABLE_NAME, S3_BUCKET_NAME};
    use crate::pages::api::tokens::validate_and_get_token_info;
    use crate::utils::server::create_aws_config;
    use aws_sdk_dynamodb::types::AttributeValue;

    let mut multipart = data.into_inner().unwrap();

    let mut file_name = String::new();
    let mut file_bytes = Vec::<u8>::new();
    let mut form_id = String::new();
    let mut access_token = String::new();
    let mut input_name = String::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or_default();

        match field_name {
            "access_token" => access_token = field.text().await?,
            "form_id" => form_id = field.text().await?,
            _ => {
                // NOTE: If there are multiple files put here the function will only upload the last
                // one in the data.
                // We should likely use chunk streaming in the future, for safety.
                input_name = field
                    .name()
                    .ok_or(ServerFnError::new("Couldn't find input name"))?
                    .to_owned();
                file_name = field
                    .file_name()
                    .ok_or(ServerFnError::new("Couldn't find file name"))?
                    .to_owned();
                file_bytes = field.bytes().await?.to_vec();
            }
        }
    }

    if access_token.is_empty() {
        return Err(ServerFnError::new("Missing access token"));
    }
    if form_id.is_empty() {
        return Err(ServerFnError::new("Missing form ID"));
    }
    if file_name.is_empty() {
        return Err(ServerFnError::new("Missing file"));
    }
    if input_name.is_empty() {
        return Err(ServerFnError::new("Missing input file"));
    }

    let user_claims = validate_and_get_token_info(access_token).await?;
    let subject = user_claims.subject;

    let key = format!("{form_id}/{subject}/{input_name}/{file_name}");

    debug_log!("Adding file to S3: {:?}", key);

    let s3_client = aws_sdk_s3::Client::new(&create_aws_config().await);

    s3_client
        .put_object()
        .bucket(S3_BUCKET_NAME)
        .key(&key)
        .body(file_bytes.into())
        .send()
        .await
        .map(|_| ())
        .map_err(|e| ServerFnError::new(format!("Couldn't put file to S3: {}", e.to_string())))?;

    // Besides uploading the file to S3, we also want to store a FILE entry in Dynamo.
    // This will allow the server to quickly return a list of all files that a specific user has
    // uploaded, or a list of all files submitted for a specific question with their keys for
    // additional access.
    let dynamo_client = aws_sdk_dynamodb::Client::new(&create_aws_config().await);

    let dynamo_result = dynamo_client
        .put_item()
        .table_name(MAIN_TABLE_NAME)
        .item(
            "HK".to_string(),
            AttributeValue::S(format!("STUDENT#{subject}")),
        )
        .item(
            "SK".to_string(),
            AttributeValue::S(format!("FILE#{form_id}#{input_name}#{file_name}")),
        )
        .item(
            "file_name".to_string(),
            AttributeValue::S(file_name.clone()),
        )
        .item("file_key".to_string(), AttributeValue::S(key.clone()))
        .send()
        .await
        .map_err(|e| {
            ServerFnError::new(format!(
                "Couldn't put file to Dynamo, file rolled back: {}",
                e.to_string()
            ))
        });

    // If Dynamo fails, we want to handle the error by rolling back the file. Then, we return a failure.
    if let Err(err) = dynamo_result {
        s3_client
            .delete_object()
            .bucket("leptos-scholarships")
            .key(&key)
            .send()
            .await
            .map_err(|e| {
                ServerFnError::new(format!(
                    "Couldn't roll back S3 operation: {}",
                    e.to_string()
                ))
            })?;

        return Err(err);
    }

    Ok(file_name)
}

#[server]
pub async fn delete_file(
    access_token: String,
    form_id: String,
    input_name: String,
    file_name: String,
) -> Result<String, ServerFnError> {
    use super::{MAIN_TABLE_NAME, S3_BUCKET_NAME};
    use crate::pages::api::tokens::validate_and_get_token_info;
    use crate::utils::server::create_aws_config;
    use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue};

    let user_claims = validate_and_get_token_info(access_token).await?;
    let subject = user_claims.subject;

    let entry_hk = format!("STUDENT#{subject}");
    let entry_sk = format!("FILE#{form_id}#{input_name}#{file_name}");

    let key = format!("{form_id}/{subject}/{input_name}/{file_name}");

    let config = create_aws_config().await;
    let s3_client = aws_sdk_s3::Client::new(&config);
    let dynamo_client = aws_sdk_dynamodb::Client::new(&config);

    let previous_values = dynamo_client
        .delete_item()
        .table_name(MAIN_TABLE_NAME)
        .key("HK".to_string(), AttributeValue::S(entry_hk))
        .key("SK".to_string(), AttributeValue::S(entry_sk))
        .return_values(ReturnValue::AllOld)
        .send()
        .await?
        .attributes
        .unwrap_or_default();

    let s3_result = s3_client
        .delete_object()
        .bucket(S3_BUCKET_NAME)
        .key(key)
        .send()
        .await
        .map_err(ServerFnError::from);

    if let Err(err) = s3_result {
        // Add the key back to Dynamo and return an error.
        dynamo_client
            .put_item()
            .table_name(MAIN_TABLE_NAME)
            .set_item(Some(previous_values))
            .send()
            .await
            .map_err(|err| {
                ServerFnError::new(format!(
                    "Couldn't roll back Dynamo operation: {}",
                    err.to_string()
                ))
            })?;

        return Err(err);
    }

    Ok(file_name)
}
