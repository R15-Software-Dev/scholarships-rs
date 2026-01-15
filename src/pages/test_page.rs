use chrono::DateTime;
use crate::common::{DateInfo, DateRange};
use crate::components::{DashboardButton, DateList};
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
                        <div class="rounded-lg shadow-lg/25 overflow-hidden">
                            <div class="bg-red-900 text-white px-4 py-3 shadow-lg">
                                <h3 class="text-white font-bold">Important Dates</h3>
                            </div>
                            <div class="p-2">
                                <div class="flex flex-col p-3 gap-3">
                                    <DateList dates=vec![
                                        DateInfo {
                                            title: "Open Example".to_string(),
                                            date: DateRange::Range(
                                                DateTime::parse_from_rfc3339("2026-01-15T07:15:00-05:00")
                                                    .unwrap_or_default(),
                                                DateTime::parse_from_rfc3339("2026-01-30T14:05:00-05:00")
                                                    .unwrap_or_default()
                                            ),
                                            description: "This is a test".to_string(),
                                        },
                                        DateInfo {
                                            title: "Closed Example".to_string(),
                                            date: DateRange::Range(
                                                DateTime::parse_from_rfc3339("2026-01-01T07:15:00-05:00")
                                                    .unwrap_or_default(),
                                                DateTime::parse_from_rfc3339("2026-01-14T14:05:00-05:00")
                                                    .unwrap_or_default()
                                            ),
                                            description: "".to_string(),
                                        },
                                        DateInfo {
                                            title: "Deadline Example".to_string(),
                                            date: DateRange::Range(
                                                DateTime::parse_from_rfc3339("2026-01-15T07:15:00-05:00")
                                                    .unwrap_or_default(),
                                                DateTime::parse_from_rfc3339("2026-01-20T14:05:00-05:00")
                                                    .unwrap_or_default()
                                            ),
                                            description: "This is a test".to_string(),
                                        },
                                        DateInfo {
                                            title: "Upcoming Example".to_string(),
                                            date: DateRange::Range(
                                                DateTime::parse_from_rfc3339("2026-01-30T07:15:00-05:00")
                                                    .unwrap_or_default(),
                                                DateTime::parse_from_rfc3339("2026-02-20T14:05:00-05:00")
                                                    .unwrap_or_default()
                                            ),
                                            description: "This is a test".to_string(),
                                        },
                                        DateInfo {
                                            title: "Single Date (Blank)".to_string(),
                                            date: DateRange::Single(
                                                DateTime::parse_from_rfc3339("2026-01-15T14:05:00-05:00")
                                                    .unwrap_or_default()
                                            ),
                                            description: "".to_string(),
                                        }
                                    ] />
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </div>
        </div>
    }
}
