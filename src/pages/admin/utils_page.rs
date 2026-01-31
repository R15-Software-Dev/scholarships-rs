use chrono::DateTime;
use crate::components::ActionButton;
use leptos::prelude::*;
use crate::common::{DateInfo, DateRange};
use crate::pages::api::{CreateDates, CreateTestComparisons};

fn get_important_dates() -> Vec<DateInfo> {
    vec! [
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Provider Forms Open".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-01-20T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-03-03T23:59:00-05:00").unwrap()
            ),
            description: "".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Student Forms Open".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-03-03T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-03-27T14:05:00-05:00").unwrap()
            ),
            description: "".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Scholarship Decision Window".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-04-10T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-05-01T23:59:00-05:00").unwrap()
            ),
            description: "Start date may change if available sooner.".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "PHS Scholarship Committee".to_string(),
            date: DateRange::Single(
                DateTime::parse_from_rfc3339("2026-05-18T00:00:00-05:00").unwrap(),
            ),
            description: "Time TBD".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Scholarship and Awards Night".to_string(),
            date: DateRange::Single(
                DateTime::parse_from_rfc3339("2026-06-09T00:00:00-05:00").unwrap(),
            ),
            description: "Time TBD".to_string(),
        },
    ]
}

#[component]
pub fn AdminUtilsPage() -> impl IntoView {
    let create_comparisons = ServerAction::<CreateTestComparisons>::new();
    let create_dates = ServerAction::<CreateDates>::new();

    let on_click_comparisons = move |_| {
        create_comparisons.dispatch(CreateTestComparisons{});
    };

    let on_click_dates = move |_| {
        create_dates.dispatch(CreateDates{dates: get_important_dates()});
    };

    view! {
        <div class="mx-auto px-6">
            <div class="flex flex-col gap-4">
                <div class="self-centered text-lg mt-6">"This utility page provides buttons to initialize the corresponding lists of information."</div>
                <ActionButton on:click=on_click_comparisons>"Create Comparisons"</ActionButton>
                <ActionButton on:click=on_click_dates>"Create Dates"</ActionButton>
            </div>
        </div>
    }.into_any()
}
