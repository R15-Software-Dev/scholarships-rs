use crate::components::ActionButton;
use leptos::prelude::*;
use leptos_oidc::AuthSignal;

#[component]
pub fn UnauthenticatedPage() -> impl IntoView {
    let auth = use_context::<AuthSignal>().expect("AuthSignal not present in LoginLink");
    let login_url = move || {
        auth.with(|auth| {
            auth.unauthenticated()
                .map(|unauthenticated| unauthenticated.login_url())
        })
    };

    let navigate = leptos_router::hooks::use_navigate();

    view! (
        <div class="flex h-screen justify-center items-center pb-50">
            <div class="bg-white p-8 border rounded-lg shadow-lg">
                <h3> "You're not signed in. Please sign in with your Region 15 email." </h3>
                <div class="flex pt-4 justify-center items-center">
                    <ActionButton on:click=move |_| {
                        if let Some(url) = login_url() {
                            navigate(&url, Default::default())
                        }
                    }>
                        "Sign in"
                    </ActionButton>
                </div>
            </div>
        </div>
    )
}
