use crate::components::dashboard_button::DashboardButton;
use leptos::prelude::*;
#[component]
pub fn TestPage() -> impl IntoView {
    view! {
        <div class="p-6 space-y-4">
            <DashboardButton
                title="Home Page"
                description="Navigate to Home Page"
                icon="/public/square-user.svg"
                path="/"
            />
        </div>
    }
}
