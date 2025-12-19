use crate::common::{DateInfo, DateRange};
use leptos::either::Either;
use leptos::prelude::*;
/// # Date Component
///
///
/// Example usage:
/// ```

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

#[component]
pub fn Date(
    /// Struct of title, date, and description
    #[prop(into)]
    important_dates: Vec<DateInfo>,
    /// The file path or URL of the icon displayed at the top of the button.
    #[prop(into)]
    icon: String,
    /// The status of the event (open, upcoming, deadline...)
    #[prop(into)]
    status: String,
) -> impl IntoView {
    important_dates.into_iter()
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
                         <span
                            {/* Temporary status color & design*/}
                            class="inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs font-medium w-fit whitespace-nowrap shrink-0">
                                {status.clone()}
                         </span>
                        </div>
                    </div>
                </div>
                </div>
                }
            })
            .collect_view()
}
