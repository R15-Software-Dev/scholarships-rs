use crate::common::{ADMIN_POOL_ID, COGNITO_REGION};
use crate::components::login::setup::provide_auth_context;
use leptos::prelude::*;

#[component]
pub fn AdminLoginContext(children: Children) -> impl IntoView {
    provide_auth_context(
        crate::utils::use_origin(),
        format!("https://cognito-idp.{COGNITO_REGION}.amazonaws.com/{ADMIN_POOL_ID}"),
        "1vh1q994rid6cgi6tjf58r9jp4",
        "/admin/callback",
    );

    view! { {children()} }
}
