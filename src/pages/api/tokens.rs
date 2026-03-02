#[cfg(feature = "ssr")]
use jsonwebtoken::{jwk::JwkSet, DecodingKey};
use leptos::prelude::*;
use crate::common::UserClaims;

#[cfg(feature = "ssr")]
pub async fn fetch_jwks(user_pool_id: String, region: String) -> Result<JwkSet, ServerFnError> {
    let request_url = format!(
        "https://cognito-idp.{}.amazonaws.com/{}/.well-known/jwks.json",
        region,
        user_pool_id
    );

    let jwks = reqwest::get(&request_url).await?
        .json::<JwkSet>()
        .await?;

    Ok(jwks)
}

/// Validates a JWT, and, if successful, decodes it into a series of user claims.
#[cfg(feature = "ssr")]
pub async fn validate_and_get_token_info(token: String) -> Result<UserClaims, ServerFnError> {
    use jsonwebtoken::{decode, Algorithm, Validation};

    let jwks = fetch_jwks("us-east-1_Lfjuy5zaM".into(), "us-east-1".into()).await?;

    let validation = Validation::new(Algorithm::RS256);
    // validation.set_audience(&["scholarships-rs"]);

    let header = jsonwebtoken::decode_header(&token)
        .map_err(|e| ServerFnError::new(format!("Invalid token header: {e}")))?;

    let kid = header.kid.ok_or_else(|| ServerFnError::new("Missing kid header"))?;

    let jwk = jwks.find(&kid)
        .ok_or_else(|| ServerFnError::new("Couldn't find matching key in JWKS"))?;

    let token_data = decode::<UserClaims>(
        &token,
        &DecodingKey::from_jwk(&jwk)?,
        &validation
    ).map_err(|_| ServerFnError::new("Invalid token found".to_string()))?;

    if token_data.claims.token_use != "access" {
        return Err(ServerFnError::new("Token type mismatch"));
    }

    Ok(token_data.claims)
}
