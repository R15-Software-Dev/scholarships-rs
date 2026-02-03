use crate::pages::{ProviderPortal, ScholarshipInfoPage, ProviderContactPage, LoanerShell, LoanerFallback, LoanerBorrowForm, LoanerReturnForm, AuthCallbackPage, ApplicantsPageFallback, AdminShell, AdminHomePage, AdminProviderPage, AdminScholarshipPage, AdminUtilsPage};
use crate::pages::student;
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
                    // <Route path=path!("/testing") view=TestPage />
                    <ProviderRoutes />
                    <LoanerRoutes />
                    <AdminRoutes />
                    <StudentRoutes />
                </Routes>
            </ToastList>
        </main>
    }
}

#[component(transparent)]
fn ProviderRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("/providers") view=ProviderLoginContext>
            <Route path=path!("") view=ProviderPortal />
            <Route path=path!("/applicants") view=ApplicantsPageFallback />
            <Route path=path!("/callback") view=AuthCallbackPage />
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

#[component(transparent)]
fn AdminRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("admin") view=AdminShell>
            <Route path=path!("home") view=AdminHomePage />
            <Route path=path!("providers") view=AdminProviderPage />
            <Route path=path!("scholarships") view=AdminScholarshipPage />
            <Route path=path!("utils") view=AdminUtilsPage />
        </ParentRoute>
    }
        .into_inner()
        .into_any_nested_route()
}

#[component(transparent)]
fn StudentRoutes() -> impl MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("students") view=student::StudentShell>
            <Route path=path!("callback") view=AuthCallbackPage />
            <Route path=path!("home") view=student::StudentHomePage />
            <Route path=path!("demographics") view=student::StudentDemographicsPage />
            <Route path=path!("additional") view=student::AdditionalPage />
            <Route path=path!("additional/academics") view=student::StudentAcademicsPage />
            <Route path=path!("additional/athletics") view=student::StudentAthleticsPage />
            <Route
                path=path!("additional/extracurriculars")
                view=student::StudentExtracurricularsPage
            />
            <Route
                path=path!("additional/work-experience")
                view=student::StudentWorkExperiencePage
            />
            <Route
                path=path!("additional/university")
                view=student::StudentUniversityPage
            />
            <Route
                path=path!("additional/family-info")
                view=student::StudentFamilyPage
            />
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
