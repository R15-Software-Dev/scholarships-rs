use crate::common::UserClaims;
use leptos::prelude::{Signal, With, use_context};
use leptos_oidc::{Algorithm, AuthSignal, TokenData};

/// # Server Utilities
///
/// These utility functions can only be used from the server. They are created to use server-side
/// dependencies like the AWS SDK. Use of these functions will crash the front end, as these
/// dependencies cannot be compiled for WASM (nor should they be).
#[cfg(feature = "ssr")]
pub mod server_utils {
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
}

/// Gets the current user claims. This function should only be used in an area that has
/// access to an AuthSignal, or it will result in a total failure.
pub fn get_user_claims() -> Signal<Option<TokenData<UserClaims>>> {
    let auth = use_context::<AuthSignal>().expect("Couldn't find AuthSignal.");
    Signal::derive(move || {
        auth.with(|auth| {
            auth.authenticated().and_then(|data| {
                data.decoded_access_token::<UserClaims>(Algorithm::RS256, &["account"])
            })
        })
    })
}
