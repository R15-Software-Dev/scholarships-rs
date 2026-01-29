use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use crate::common::TabInfo;
use crate::components::{Banner, Loading, TabSidebarList};
use crate::components::login::StudentLoginContext;
use crate::pages::UnauthenticatedPage;

#[component]
pub fn StudentShell() -> impl IntoView {
    // Create login context
    view! {
        <StudentLoginContext>
            <Banner title="R15 Student Scholarship Application" logo="/PHS_Stacked_Acronym.png" />
            <AuthLoaded fallback=Loading>
                <Authenticated unauthenticated=UnauthenticatedPage>
                    <TabSidebarList tabs=vec![
                        TabInfo::new("Home", "/students/home"),
                        TabInfo::new("Demographics", "/students/demographics"),
                        TabInfo::new("Additional Info", "/students/additional"),
                    ] />
                </Authenticated>
            </AuthLoaded>
        </StudentLoginContext>
    }
}
