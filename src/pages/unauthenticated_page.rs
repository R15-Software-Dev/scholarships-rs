use leptos::prelude::*;
use leptos_oidc::{LoginLink};
use crate::components::ActionButton;

#[component]
pub fn UnauthenticatedPage() -> impl IntoView {
    view! (
        <div class="flex h-screen justify-center items-center pb-50">
            <div class="bg-white p-8 border rounded-lg shadow-lg">
                <h3> "You're not signed in. Please sign in with your Region 15 email." </h3>
                <div class="flex pt-4 justify-center items-center">
                    <LoginLink class="text-login">
                        <ActionButton> "Sign in" </ActionButton>
                    </LoginLink>
                </div>
            </div>
        </div>

    )
}