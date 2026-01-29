use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use crate::common::{SubTabInfo, TabInfo};
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
                        TabInfo::new("Home", "/students/home", None),
                        TabInfo::new("Demographics", "/students/demographics",
                            Some(vec![
                                SubTabInfo::new("Testing", "/students/testing"),
                                SubTabInfo::new("Testing Again!", "/students/testing/again")
                            ])),
                        TabInfo::new("Additional Info", "/students/additional", 
                            Some(vec![
                                SubTabInfo::new("Testing", "/students/testing"),
                                SubTabInfo::new("Testing Again!", "/students/testing/again")
                            ])),
                    ] />
                </Authenticated>
            </AuthLoaded>
        </StudentLoginContext>
    }
}
