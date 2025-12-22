use crate::pages::{AboutPage, ComparisonTestPage, HomePage, LoanerPage, ProviderPortal, TestPage};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_oidc::{Auth, AuthParameters, AuthSignal, Challenge};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

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
        <Stylesheet id="leptos" href="/pkg/scholarships-rs-wasm.css"/>

        // sets the document title
        <Title text="R15 Scholarship App DEV"/>

        // content for this welcome page
        <Router>
            <AppWithRoutes />
        </Router>
    }
}

fn use_origin() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::web_sys;
        // Just get the current URL origin.
        web_sys::window()
            .unwrap()
            .location()
            .origin()
            .unwrap_or("http://localhost:3000/".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Read the expected origin out of an environment variable.
        std::env::var("LP_SITE_ORIGIN").unwrap_or("http://localhost:3000/".to_string())
    }
}

#[component]
pub fn AppWithRoutes() -> impl IntoView {
    provide_meta_context();

    let current_origin = use_origin();
    log!("Current origin: {}", current_origin);

    // Authentication setup
    let parameters = AuthParameters {
        issuer: "https://cognito-idp.us-east-1.amazonaws.com/us-east-1_Lfjuy5zaM".into(),
        client_id: "10jr2h3vtpu9n7gj46pvg5qo2q".into(),
        redirect_uri: format!("{}/", use_origin()),
        // redirect_uri: Url::parse(format!("{}", current_origin).as_str()).unwrap().to_string(),
        // redirect_uri: Url::parse("http://localhost:3000").unwrap().to_string(),
        post_logout_redirect_uri: current_origin,
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

        <main>
            // TODO Create a 404 page
            <Routes fallback=|| "Page not found.".into_view()>
                <Route path=path!("") view=HomePage/>
                <Route path=path!("about") view=AboutPage/>
                <Route path=path!("test_page") view=TestPage/>
                <Route path=path!("comparison") view=ComparisonTestPage />
                <Route path=path!("providers") view=ProviderPortal />
                <Route path=path!("loaners") view=LoanerPage />
                <Route path=path!("loaners/:form_name") view=LoanerPage />
            </Routes>
        </main>
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
