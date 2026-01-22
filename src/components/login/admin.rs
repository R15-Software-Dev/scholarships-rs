use leptos::prelude::*;
use crate::components::login::setup::provide_auth_context;

#[component]
pub fn AdminLoginContext(
    children: Children
) -> impl IntoView {
    provide_auth_context(
        crate::utils::use_origin(),
        "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_Lfjuy5zaM",
        "56c2bqvl021rv8d5mq36blt7jv",
        "/providers/callback"
    );

    view! {
        {children()}
    }
}