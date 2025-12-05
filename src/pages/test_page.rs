use crate::components::DashboardButton;
use leptos::prelude::*;

#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-4 max-w-1/3">
            <DashboardButton
                title="Home Page"
                description="Navigate to Home Page"
                icon="Book.png"
                path="/"
            />
        <DashboardButton
                title="Home Page"
                description="Navigate to Home Page"
                icon="Calendar.png"
                path="/"
            />
        <DashboardButton
                title="Home Page"
                description="Navigate to Home Page"
                icon="Edit.png"
                path="/"
            />
        </div>
    }
}
