use leptos::prelude::*;
use crate::common::TabInfo;
use crate::components::{Banner, TabSidebarList};
use crate::components::login::AdminLoginContext;

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
            // We'll put the tabs here.
            <TabSidebarList
                tabs=vec![
                    TabInfo {
                        name: "Home".to_string(),
                        path: "/admin/home".to_string(),
                    },
                    TabInfo {
                        name: "Test".to_string(),
                        path: "/admin/testing".to_string(),
                    }
                ]
            />
        </AdminLoginContext>
    }
}

#[component]
pub fn AdminFallback() -> impl IntoView {
    view! {
        <div>
            <p class="mx-auto">"This is a fallback page"</p>
        </div>
    }
}

#[component]
pub fn AdminTesting() -> impl IntoView {
    view! {
        <div>
            <p class="mx-auto">"This is a testing page"</p>
        </div>
    }
}
