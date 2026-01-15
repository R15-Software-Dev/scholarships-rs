use std::collections::HashMap;
use aws_config::SdkConfig;
use crate::common::ValueType;

/// Creates an [`SdkConfig`] struct for use with AWS SDK structs.
pub async fn create_aws_config() -> SdkConfig {
    aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await
}

/// Creates a DynamoDB client.
pub async fn create_dynamo_client() -> aws_sdk_dynamodb::Client {
    aws_sdk_dynamodb::Client::new(&create_aws_config().await)
}

pub fn into_attr_map(map: HashMap<String, ValueType>) -> HashMap<String, aws_sdk_dynamodb::types::AttributeValue> {
    map.into_iter().map(|(k, v)| (k, v.into())).collect()
}