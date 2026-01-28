use leptos::prelude::*;
use crate::common::TabInfo;
use crate::components::{Banner, TabSidebarList};
use crate::components::login::StudentLoginContext;

#[component]
pub fn StudentShell() -> impl IntoView {
    // Create login context
    view! {
        <StudentLoginContext>
            <Banner title="R15 Student Scholarship Application" logo="/PHS_Stacked_Acronym.png" />
            // We'll put the tabs here.
            <TabSidebarList tabs=vec![
                TabInfo::new("Home", "/students/home"),
                TabInfo::new("Demographics", "/students/demographics"),
                TabInfo::new("Additional Info", "/students/additional"),
            ] />
        </StudentLoginContext>
    }
}
