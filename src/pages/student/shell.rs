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
            <div class="h-screen flex flex-col overflow-hidden">
                <Banner
                    title="R15 Student Scholarship Application"
                    logo="/PHS_Stacked_Acronym.png"
                />
            
                <div class="flex flex-1 min-h-0">
                    <AuthLoaded fallback=Loading>
                        <Authenticated unauthenticated=UnauthenticatedPage>
                            <TabSidebarList
                                base_path="students"
                                tabs=vec![
                                    TabInfo::new("Home", "home", None),
                                    TabInfo::new("Demographics", "demographics", None),
                                    TabInfo::new(
                                        "Additional Info",
                                        "additional",
                                        Some(
                                            vec![
                                                SubTabInfo::new("Academics", "academics"),
                                                SubTabInfo::new("Athletics", "athletics"),
                                                SubTabInfo::new("Work Experience", "work-experience"),
                                                SubTabInfo::new("Extracurriculars", "extracurriculars"),
                                                SubTabInfo::new("University Info", "university"),
                                                SubTabInfo::new("Family Info", "family-info"),
                                            ],
                                        ),
                                    ),
                                ]
                            />
                        </Authenticated>
                    </AuthLoaded>
                </div>
            </div>
        </StudentLoginContext>
    }
}
