use leptos::prelude::*;
use leptos_router::components::Outlet;
use super::setup::provide_auth_context;

///# Student Login Context Component
/// 
/// This component handles providing context to the student login information.
/// It creates the corresponding [`AuthSignal`], specific to the student login page and redirects.
/// 
/// Example usage:
/// ```
/// view! {
///     <StudentLoginContext>
///         // The rest of the student application. Don't forget AuthLoaded and 
///         // Authenticated components!
///     </StudentLoginContext>
/// }
#[component]
pub fn StudentLoginContext() -> impl IntoView {
    provide_auth_context(
        crate::utils::use_origin(),
        "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_Lfjuy5zaM",
        "10jr2h3vtpu9n7gj46pvg5qo2q",
        "/students/callback"
    );
    
    view! { <Outlet /> }
}
