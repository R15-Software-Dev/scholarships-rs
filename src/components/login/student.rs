use leptos::prelude::*;
use super::setup::provide_auth_context;
use crate::common::{COGNITO_REGION, STUDENT_PROVIDER_POOL_ID};

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
pub fn StudentLoginContext(
    children: Children,
) -> impl IntoView {
    provide_auth_context(
        crate::utils::use_origin(),
        format!("https://cognito-idp.{COGNITO_REGION}.amazonaws.com/{STUDENT_PROVIDER_POOL_ID}"),
        "10jr2h3vtpu9n7gj46pvg5qo2q",
        "/students/callback"
    );
    
    view! { {children()} }
}
