use leptos::prelude::*;
use server_fn::codec::{MultipartFormData, MultipartData};
use leptos::logging::debug_log;

#[server(input = MultipartFormData)]
pub async fn upload_file(
    data: MultipartData
) -> Result<(), ServerFnError> {
    use crate::pages::api::tokens::validate_and_get_token_info;
    use crate::utils::server::create_aws_config;

    let mut multipart = data.into_inner().unwrap();

    let mut file_name = String::new();
    let mut file_bytes = Vec::<u8>::new();
    let mut form_id = String::new();
    let mut access_token = String::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or_default();

        match field_name {
            "access_token" => access_token = field.text().await?,
            "form_id" => form_id = field.text().await?,
            _ => {
                // NOTE: If there are multiple files put here the function will only upload the last
                // one in the data.
                // We should likely use chunk streaming in the future, for safety.
                file_name = field.file_name()
                    .ok_or(ServerFnError::new("Couldn't find file name"))?
                    .to_owned();
                file_bytes = field.bytes().await?.to_vec();
            },
        }
    }

    if access_token.is_empty() { return Err(ServerFnError::new("Missing access token")); }
    if form_id.is_empty() { return Err(ServerFnError::new("Missing form ID")); }
    if file_name.is_empty() { return Err(ServerFnError::new("Missing file")); }

    let user_claims = validate_and_get_token_info(access_token).await?;
    let subject = user_claims.subject;

    let key = format!("{form_id}/{subject}/{file_name}");

    debug_log!("Adding file to S3: {:?}", key);

    let client = aws_sdk_s3::Client::new(&create_aws_config().await);

    client
        .put_object()
        .bucket("leptos-scholarships")
        .key(key)
        .body(file_bytes.into())
        .send()
        .await
        .map(|_| ())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_file(
    access_token: String,
    form_id: String,
    file_name: String
) -> Result<(), ServerFnError> {
    use crate::utils::server::create_aws_config;
    use crate::pages::api::tokens::validate_and_get_token_info;

    let user_claims = validate_and_get_token_info(access_token).await?;
    let subject = user_claims.subject;

    let key = format!("{form_id}/{subject}/{file_name}");

    let client = aws_sdk_s3::Client::new(&create_aws_config().await);

    client
        .delete_object()
        .bucket("leptos-scholarhips")
        .key(key)
        .send()
        .await
        .map(|_| ())
        .map_err(ServerFnError::from)
}
