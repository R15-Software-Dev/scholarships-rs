use crate::common::TabInfo;
use crate::components::login::AdminLoginContext;
use crate::components::{Banner, Loading, TabSidebarList};
use crate::pages::UnauthenticatedPage;
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};

/// # Admin Shell Component
///
/// This shell should keep track of pretty much everything that changes across the admin page. It
/// will provide the login context, the banner, and the tabs.
#[component]
pub fn AdminShell() -> impl IntoView {
    // Create login context
    view! {
        <AdminLoginContext>
            <Banner title="R15 Scholarships Admin" logo="/PHS_Stacked_Acronym.png" />
            <AuthLoaded fallback=Loading>
                <Authenticated unauthenticated=UnauthenticatedPage>
                    // We'll put the tabs here.
                    <TabSidebarList
                        base_path="admin"
                        tabs=vec![
                            TabInfo::new("Home", "home", None),
                            TabInfo::new("Providers", "providers", None),
                            TabInfo::new("Scholarships", "scholarships", None),
                            TabInfo::new("Utilities", "utilities", None),
                            TabInfo::new("Scholarship Applicants", "applicants", None),
                        ]
                    />
                </Authenticated>
            </AuthLoaded>
        </AdminLoginContext>
    }
}
