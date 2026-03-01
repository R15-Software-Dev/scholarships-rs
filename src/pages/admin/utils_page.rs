use crate::common::{DateInfo, DateRange};
use crate::components::ActionButton;
use crate::pages::api::exports::get_scholarship_csv;
use crate::pages::api::{CreateDates, CreateTestComparisons, exports::GetScholarshipCsv};
use chrono::DateTime;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::web_sys::{Blob, HtmlAnchorElement, Url, js_sys};

fn get_important_dates() -> Vec<DateInfo> {
    vec![
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Provider Forms Open".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-01-20T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-03-03T23:59:00-05:00").unwrap(),
            ),
            description: "".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Student Forms Open".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-03-03T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-03-27T14:05:00-05:00").unwrap(),
            ),
            description: "".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Scholarship Decision Window".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-04-10T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-05-01T23:59:00-05:00").unwrap(),
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
    let create_export = ServerAction::<GetScholarshipCsv>::new();

    let on_click_comparisons = move |_| {
        create_comparisons.dispatch(CreateTestComparisons {});
    };

    let on_click_dates = move |_| {
        create_dates.dispatch(CreateDates {
            dates: get_important_dates(),
        });
    };

    let on_click_export = move |_| {
        spawn_local(async move {
            let file_bytes = get_scholarship_csv().await.unwrap_or_default();

            let uint_arr = js_sys::Uint8Array::new_with_length(file_bytes.len() as u32);
            uint_arr.copy_from(&file_bytes);

            let blob_parts = js_sys::Array::new();
            blob_parts.push(&uint_arr);

            let blob = Blob::new_with_u8_array_sequence(&blob_parts).unwrap();

            let blob_url = Url::create_object_url_with_blob(&blob).unwrap();

            let window = window();
            let document = window.document().unwrap();
            let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();

            a.set_href(&blob_url);
            a.set_download("scholarships.csv");
            document.body().unwrap().append_child(&a).unwrap();
            a.click();

            document.body().unwrap().remove_child(&a).unwrap();
            Url::revoke_object_url(&blob_url).unwrap();
        });
    };

    view! {
        <div class="mx-auto px-6">
            <div class="flex flex-col gap-4">
                <div class="self-centered text-lg mt-6">
                    "This utility page provides buttons to initialize the corresponding lists of information."
                </div>
                <ActionButton on:click=on_click_comparisons>"Create Comparisons"</ActionButton>
                <ActionButton on:click=on_click_dates>"Create Dates"</ActionButton>
                <ActionButton on:click=on_click_export>"Scholarship Export"</ActionButton>
            </div>
        </div>
    }.into_any()
}
