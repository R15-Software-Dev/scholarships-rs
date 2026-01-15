use leptos::prelude::*;
use leptos_oidc::AuthSignal;
use leptos_router::hooks::use_navigate;

#[component]
pub fn AuthCallbackPage() -> impl IntoView {
    // This should wait for authentication, read the values in session storage, and then redirect
    // to the correct page. The session storage should contain a path, which we'll need to check.
    
    let auth = expect_context::<AuthSignal>();
    let navigate = use_navigate();
    
    Effect::new(move || {
        // Wait for auth to load, then read information.
        if let Some(_) = auth.get().authenticated() {
            // Read the sessionStorage to get the last navigated path.
            let storage_opt = window().session_storage().ok().flatten();
            let path = storage_opt
                .and_then(|storage| storage.get_item("post_login_path").ok().flatten())
                .unwrap_or_else(|| "/".to_string());
            
            // Clear storage in case we need to reauth again
            if let Some(storage) = window().session_storage().ok().flatten() {
                let _ = storage.remove_item("post_login_path");
            }
            
            // Redirect to the stored path
            navigate(&path, Default::default());
        }
    });
    
    view! {
        <div class="flex flex-col w-full mx-auto items-center justify-center">
            <p>"Completing the login process..."</p>
        </div>
    }
}
