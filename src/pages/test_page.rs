use chrono::DateTime;
use crate::common::{DateInfo, DateRange};
use crate::components::{ActionButton, DashboardButton, DateList};
use leptos::prelude::*;
use crate::pages::api::{get_important_dates, CreateDates};

#[component]
pub fn TestPage() -> impl IntoView {    
    view! {
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
                            path="/"
                        />
                        <DashboardButton
                            title="Applicants"
                            description="View Scholarship Applicants"
                            icon="/Form_Black.png"
                            path="/"
                        />
                    </section>
                    <section class="space-y-2">
                        <div class="rounded-lg shadow-lg/25 overflow-hidden">
                            <div class="bg-red-900 text-white px-4 py-3 shadow-lg">
                                <h3 class="text-white font-bold">Important Dates</h3>
                            </div>
                            <div class="p-2">
                                <div class="flex flex-col p-3 gap-3">
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}
