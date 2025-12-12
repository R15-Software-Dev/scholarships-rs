/// # Server Utilities
/// 
/// These utility functions can only be used from the server. They are created to use server-side
/// dependencies like the AWS SDK. Use of these functions will crash the front end, as these
/// dependencies cannot be compiled for WASM (nor should they be).
#[cfg(feature = "ssr")]
pub mod server_utils {
    use aws_config::SdkConfig;

    /// Creates an [`SdkConfig`] struct for use with AWS SDK structs.
    pub async fn create_aws_config() -> SdkConfig {
        aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await
    }
    
    /// Creates a DynamoDB client.
    pub async fn create_dynamo_client() -> aws_sdk_dynamodb::Client {
        aws_sdk_dynamodb::Client::new(&create_aws_config().await)
    }
}
