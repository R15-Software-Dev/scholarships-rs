use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_oidc::{Auth, AuthParameters, AuthSignal, Challenge};
use leptos_router::{components::{Route, Router, Routes}, path};
use url::Url;

use crate::pages::{AboutPage, ComparisonTestPage, HomePage, ScholarshipInfoPage};

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

    // Authentication setup
    let parameters = AuthParameters {
        issuer: "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_Lfjuy5zaM".into(),
        client_id: "10jr2h3vtpu9n7gj46pvg5qo2q".into(),
        redirect_uri: Url::parse("http://localhost:3000").unwrap().to_string(),
        post_logout_redirect_uri: Url::parse("http://localhost:3000").unwrap().to_string(),
        scope: Some("openid%20profile%20email".into()),
        audience: None,
        challenge: Challenge::None,
    };

    let auth = Auth::signal();
    provide_context(auth); // allows use of this signal in lower areas of the tree without
                           // explicitly passing it through the html tree

    let _ = Auth::init(parameters);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/scholarships-rs-wasm.css" />

        // <AuthLoading><p>"Authentication is loading"</p></AuthLoading>
        // <AuthErrorContext><AuthErrorView/></AuthErrorContext>

        // sets the document title
        <Title text="R15 Scholarship App DEV" />

        // content for this welcome page
        <Router>
            <main>
                // TODO Create a 404 page
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("about") view=AboutPage />
                    <Route path=path!("comparison") view=ComparisonTestPage />
                    <Route path=path!("scholarship/:id") view=ScholarshipInfoPage />
                </Routes>
            </main>
        </Router>
    }
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
