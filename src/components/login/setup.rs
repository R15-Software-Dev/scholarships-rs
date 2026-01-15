use leptos::prelude::*;
use leptos_oidc::{Auth, AuthParameters, Challenge};

pub fn provide_auth_context(
    origin: String,
    issuer: impl Into<String>,
    client_id: impl Into<String>,
    redirect_path: impl Into<String>,
) {
    let params = AuthParameters {
        issuer: issuer.into(),
        client_id: client_id.into(),
        redirect_uri: format!("{}{}", origin, redirect_path.into()),
        post_logout_redirect_uri: origin,
        scope: Some("openid%20profile%20email".into()),
        audience: None,
        challenge: Challenge::None,
    };
    
    let auth = Auth::signal();
    provide_context(auth);
    
    let _ = Auth::init(params);
}
