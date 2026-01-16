use crate::pages::{AboutPage, HomePage, ProviderPortal, ComparisonTestPage, ScholarshipInfoPage, TestPage, ProviderContactPage, LoanerShell, LoanerFallback, LoanerBorrowForm, LoanerReturnForm, AuthCallbackPage};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_oidc::AuthSignal;
use leptos_router::{components::{Route, Router, Routes, ParentRoute}, path, MatchNestedRoutes};
use leptos_router::any_nested_route::IntoAnyNestedRoute;
use crate::components::login::ProviderLoginContext;
use crate::components::ToastList;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/scholarships-rs-wasm.css" />

        // sets the document title
        <Title text="R15 Scholarship App DEV" />

        // content for this welcome page
        <Router>
            <AppWithRoutes />
        </Router>
    }
}

#[component]
pub fn AppWithRoutes() -> impl IntoView {
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/scholarships-rs-wasm.css" />

        // sets the document title
        <Title text="R15 Scholarship App DEV" />

        <main>
            <ToastList>
                // TODO Create a 404 page
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("/about") view=AboutPage />
                    <Route path=path!("/test_page") view=TestPage />
                    <Route path=path!("/comparison") view=ComparisonTestPage />
                    <ProviderRoutes />
                    <LoanerRoutes />
                </Routes>
            </ToastList>
        </main>
    }
}

#[component(transparent)]
fn ProviderRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/providers") view=ProviderLoginContext>
            <Route path=path!("/callback") view=AuthCallbackPage />
            <Route path=path!("") view=ProviderPortal />
            <Route path=path!("/profile") view=ProviderContactPage />
            <ParentRoute path=path!("/scholarships") view=ScholarshipInfoPage>
                <Route path=path!(":id") view=ScholarshipInfoPage />
                <Route path=path!("") view=ScholarshipInfoPage />
            </ParentRoute>
        </ParentRoute>
    }
        .into_inner()
        .into_any_nested_route()
}

#[component(transparent)]
fn LoanerRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("loaners") view=LoanerShell>
            <Route path=path!("") view=LoanerFallback />
            <Route path=path!("borrowing") view=LoanerBorrowForm />
            <Route path=path!("returning") view=LoanerReturnForm />
        </ParentRoute>
    }
        .into_inner()
        .into_any_nested_route()
}

#[component]
pub fn AuthErrorView() -> impl IntoView {
    let auth = use_context::<AuthSignal>().expect("AuthError: RwSignal<AuthStore> was not found");

    let error_message = move || auth.get().error().map(|error| format!("{error:?}"));

    view! {
        <h1>"An authentication error occurred."</h1>
        {error_message}
    }
}
