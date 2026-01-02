use crate::common::{DateInfo, DateRange, DateStatus};
use crate::components::{DashboardButton, Date};
use leptos::prelude::*;

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
                        <div class="rounded-lg shadow-lg overflow-hidden">
                            <div class="bg-red-900 text-white px-4 py-3 shadow-lg \
                                        shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3)">
                                <h3 class="text-white font-bold">Important Dates</h3>
                            </div>
                            <div class="p-6 space-y-6">
                                <div class="bg-white rounded-md p-3 space-y-2">
                                    <Date
                                        important_dates=vec!{
                                            DateInfo {
                                                title: "Title".to_string(),
                                                date: DateRange::Range("February 2nd, 2026".to_string(), "February 10th, 2026".to_string()),
                                                description: "This is a test".to_string(),
                                                status: DateStatus::Upcoming
                                            }
                                        }
                                        icon="/Check_Style2.png"

                                    />
                                    <Date
                                        important_dates=vec!{
                                            DateInfo {
                                                title: "Scholarships Due".to_string(),
                                                date: DateRange::Single("November 4th, 2025".to_string()),
                                                description: "This is a test".to_string(),
                                                status: DateStatus::Open
                                            }
                                        }
                                        icon="/Warning_Style2.png"
                                    />
                                    <Date
                                        important_dates=vec!{
                                            DateInfo {
                                                title: "Scholarships Due".to_string(),
                                                date: DateRange::Single("November 4th, 2025".to_string()),
                                                description: "This is a test".to_string(),
                                                status: DateStatus::Deadline
                                            }
                                        }
                                        icon="/Warning_Style2.png"
                                    />
                                    <Date
                                        important_dates=vec!{
                                            DateInfo {
                                                title: "Scholarships Due".to_string(),
                                                date: DateRange::Single("November 4th, 2025".to_string()),
                                                description: "This is a test".to_string(),
                                                status: DateStatus::Closed
                                            }
                                        }
                                        icon="/Warning_Style2.png"
                                    />
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}
