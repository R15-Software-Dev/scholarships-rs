use chrono::DateTime;
use crate::common::{DateInfo, DateRange};
use crate::components::{ActionButton, DashboardButton, DateList};
use leptos::prelude::*;
use crate::pages::api::{get_important_dates, CreateDates};

#[component]
pub fn TestPage() -> impl IntoView {
    let reload_signal = RwSignal::new(0);
    
    let dates_resource = Resource::new(
        move || reload_signal.get(),
        async move |_| {
            get_important_dates().await
        }
    );
    
    let dates_list = RwSignal::new(Vec::new());
    
    Effect::new(move || {
        if let Some(Ok(mut dates)) = dates_resource.get() {
            dates.sort_by_key(|info| info.date.get_status());
            dates_list.set(dates);
        }
    });
    
    let create_dates = ServerAction::<CreateDates>::new();
    
    let on_click = move |_| {
        let dates_list = vec![
            DateInfo {
                id: uuid::Uuid::new_v4().to_string(),
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
                id: uuid::Uuid::new_v4().to_string(),
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
                id: uuid::Uuid::new_v4().to_string(),
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
                id: uuid::Uuid::new_v4().to_string(),
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
                id: uuid::Uuid::new_v4().to_string(),
                title: "Single Date (Blank)".to_string(),
                date: DateRange::Single(
                    DateTime::parse_from_rfc3339("2026-01-15T14:05:00-05:00")
                        .unwrap_or_default()
                ),
                description: "".to_string(),
            }
        ];
        
        create_dates.dispatch(CreateDates {
            dates: dates_list
        });
    };
    
    Effect::new(move || {
        if let Some(Ok(_)) = create_dates.value().get() {
            create_dates.clear();
            reload_signal.update(|v| *v += 1);
        }
    });
    
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
                                    <DateList dates=dates_list />
                                </div>
                            </div>
                        </div>
                    </section>
                    <section>
                        <ActionButton on:click=on_click>"Create Dates List"</ActionButton>
                    </section>
                </div>
            </div>
        </div>
    }
}
