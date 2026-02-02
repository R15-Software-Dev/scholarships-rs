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
                base_path="admin"
                tabs=vec![
                    TabInfo::new("Home", "home", None),
                    TabInfo::new("Providers", "providers", None),
                    TabInfo::new("Scholarships", "scholarships", None),
                    TabInfo::new("Utilities", "utilities", None),
                ]
            />
        </AdminLoginContext>
    }
}
