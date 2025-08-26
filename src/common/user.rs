use serde::{Deserialize, Serialize};

// API URL to get user information from Google's Groups API:
// https://admin.googleapis.com/admin/directory/v1/groups?domain=region15.org&userKey=<email>
// Note that this requires a service account of some type, and that it should only be called from
// a pre-sign up lambda function.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClaims {
    /// The time the JWT was issued, in UTC time.
    #[serde(rename = "iat")]
    issued_at: usize,
    /// The issuer of the JWT.
    #[serde(rename = "iss")]
    issuer: String,
    /// The time the JWT expires, in UTC time.
    #[serde(rename = "exp")]
    expiration_at: usize,
    /// The subject of the JWT. This is the only unique identifier for a user.
    #[serde(rename = "sub")]
    pub subject: String,
    /// The username of the user. This should be their email address.
    #[serde(rename = "username")]
    pub username: String,
}
