use crate::components::login::setup::provide_auth_context;
use leptos::prelude::*;

#[component]
pub fn AdminLoginContext(children: Children) -> impl IntoView {
    provide_auth_context(
        crate::utils::use_origin(),
        "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_rvCU4Xy4j",
        "1vh1q994rid6cgi6tjf58r9jp4",
        "/admin/callback",
    );

    view! { {children()} }
}
