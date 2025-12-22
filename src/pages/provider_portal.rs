use crate::components::{Banner, DashboardButton};
use leptos::prelude::*;

#[component]
pub fn ProviderPortal() -> impl IntoView {
    view! {
        <Banner
            title="Provider Dashboard"
            logo="PHS_Stacked_Acronym.png"
            path="/"
        />
        <div class="mx-auto px-6">
            <div class="container mx-auto px-6 py-8 space-y-8">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <section class="space-y-4">
                        <DashboardButton
                            title="Profile"
                            description="Edit user profile"
                            icon="/Person_Black.png"
                            path="/"
                            />
                        <DashboardButton
                            title="Create Scholarship"
                            description="Navigate to Scholarship Creation Page"
                            icon="/Create_Black.png"
                            path="/providers/scholarships"
                            />
                        <DashboardButton
                            title="Applicants"
                            description="View Scholarship Applicants"
                            icon="/Form_Black.png"
                            path="/"
                            />
                    </section>
                    <section class="space-y-4">

                    </section>
                </div>
            </div>
        </div>
    }
}
