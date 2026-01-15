use leptos::prelude::*;
use leptos_router::components::Outlet;
use super::setup::provide_auth_context;

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
        "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_Lfjuy5zaM",
        "56c2bqvl021rv8d5mq36blt7jv",
        "/providers/callback"
    );

    view! {
        <Outlet />
    }
}
