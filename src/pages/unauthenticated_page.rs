use crate::components::ActionButton;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_oidc::AuthSignal;
use leptos_router::hooks::{use_navigate, use_location};

#[component]
pub fn UnauthenticatedPage() -> impl IntoView {
    let auth = use_context::<AuthSignal>().expect("AuthSignal not present in LoginLink");
    let login_url = Memo::new(move |_| {
        auth.with(|auth| {
            auth.unauthenticated()
                .map(|unauthenticated| unauthenticated.login_url())
        })
    });
    
    Effect::new(move || log!("Generated login url: {:?}", login_url.get()));

    let navigate = use_navigate();
    let location = use_location();
    
    let on_click = move |_| {
        let current_path = location.pathname.get();
        
        let storage_opt = window().session_storage().ok().flatten();
        if let Some(session_storage) = storage_opt {
            let _ = session_storage.set_item("post_login_path", &current_path);
        }
        if let Some(url) = login_url.get() {
            navigate(&url, Default::default())
        }
    };

    view! {
        <div class="flex mx-auto h-screen justify-center items-center pb-50">
            <div class="bg-white p-8 border rounded-lg shadow-lg">
                <h3>"You're not signed in. Please sign in first."</h3>
                <div class="flex pt-4 justify-center items-center">
                    <ActionButton on:click=on_click>"Sign in"</ActionButton>
                </div>
            </div>
        </div>
    }
}
