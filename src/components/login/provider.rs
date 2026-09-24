use leptos::prelude::*;
use leptos_router::components::Outlet;
use super::setup::provide_auth_context;
use crate::common::{COGNITO_REGION, STUDENT_PROVIDER_POOL_ID};

///# Provider Login Context Component
/// 
/// This component handles providing context to the provider login information.
/// It creates the corresponding [`AuthSignal`], specific to the student login page and redirects.
/// 
/// Example usage:
/// ```
/// view! {
///     <ProviderLoginContext>
///         // The rest of the student application. Don't forget AuthLoaded and 
///         // Authenticated components!
///     </ProviderLoginContext>
/// }
#[component]
pub fn ProviderLoginContext() -> impl IntoView {
    provide_auth_context(
        crate::utils::use_origin(),
        format!("https://cognito-idp.{COGNITO_REGION}.amazonaws.com/{STUDENT_PROVIDER_POOL_ID}"),
        "56c2bqvl021rv8d5mq36blt7jv",
        "/providers/callback"
    );

    view! { <Outlet /> }
}
