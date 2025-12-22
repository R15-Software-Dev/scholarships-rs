use crate::common::{DateInfo, DateRange, DateStatus};
use leptos::either::{Either, EitherOf4};
use leptos::prelude::*;
/// # Date Component
///
///
/// Example usage - Single Date:
/// ```
/// <Date
/// important_dates=vec!{
///     DateInfo {
///        title: "Some Title".to_string(),
///         date: DateRange::Single("November 4th, 2025".to_string()),
///         description: "This is a test".to_string(),
///         status: DateStatus::Closed
///     }
/// }
/// icon="/icon.png"
/// />
/// ```
/// Example usage - Date Range
/// ```
/// <Date
/// important_dates=vec!{
///     DateInfo {
///         title: "Some Title".to_string(),
///         date: DateRange::Range("February 2nd, 2026".to_string(), "February 10th, 2026".to_string()),
///         description: "This is a test".to_string(),
///         status: DateStatus::Upcoming
///     }
/// }
/// icon="/icon.png"
/// />
/// ```

fn render_dates(dates: DateRange) -> impl IntoView {
    match dates {
        DateRange::Single(date) => Either::Left(view! {
            <p> {date} </p>
        }),
        DateRange::Range(start_date, end_date) => Either::Right(view! {
            <p> {start_date} " - " {end_date} </p>
        }),
    }
}

fn render_status(status: DateStatus) -> impl IntoView {
    match status {
        DateStatus::Upcoming => EitherOf4::A(view! {
         <span
            class="inline-flex items-center justify-center rounded-md border border-yellow-100 bg-yellow-100 px-2 py-0.5 text-xs text-yellow-800 font-medium w-fit whitespace-nowrap shrink-0">
                Upcoming
         </span>
        }),
        DateStatus::Open => EitherOf4::B(view! {
         <span
            class="inline-flex items-center justify-center rounded-md border border-green-100 bg-green-100 px-2 py-0.5 text-xs text-green-800 font-medium w-fit whitespace-nowrap shrink-0">
                Open
         </span>
        }),
        DateStatus::Deadline => EitherOf4::C(view! {
        <span
            class="inline-flex items-center justify-center rounded-md border border-red-900 bg-red-900 px-2 py-0.5 text-xs text-white font-medium w-fit whitespace-nowrap shrink-0">
                Deadline
         </span>
        }),
        DateStatus::Closed => EitherOf4::D(view! {
             <span
                class="inline-flex items-center justify-center rounded-md border border-red-50 bg-red-100 px-2 py-0.5 text-xs text-red-800 font-medium w-fit whitespace-nowrap shrink-0">
                    Closed
             </span>
        }),
    }
}
#[component]
pub fn Date(
    /// Struct of title, date, and description
    #[prop(into)]
    important_dates: Vec<DateInfo>,
    /// The file path or URL of the icon displayed at the top of the button.
    #[prop(into)]
    icon: String,
) -> impl IntoView {
    important_dates
        .into_iter()
        .map(|info: DateInfo| {
            view! {
            <div class="dashboard-button flex items-start gap-3 rounded-lg border-grey-300 p-3
                       hover:bg-gray-100 transition w-full text-left
                       shadow-[inset_0_0_6px_rgba(0,0,0,0.12)]">
            <div class="flex-shrink-0 mt-0.5">
                <img src={icon.clone()} class="h-4 w-4" alt="icon"/>
            </div>
            <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between">
                    <div class="flex-1">
                        <h3 class="font-bold text-md text-base">
                            {info.title}
                        </h3>
                        <h3 class="text-md text-base mt-1">
                            {render_dates(info.date)}
                        </h3>
                        <p class="text-sm text-gray-600">
                            {info.description}
                        </p>
                    </div>
                    <div class="flex-shrink-0 ml-2">
                        {render_status(info.status)}
                    </div>
                </div>
            </div>
            </div>
            }
        })
        .collect_view()
}
