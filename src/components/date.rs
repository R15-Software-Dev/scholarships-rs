use crate::common::{DateInfo, DateRange, DateStatus};
use leptos::prelude::*;

fn render_dates(dates: DateRange) -> impl IntoView {
    let date_text = match dates {
        DateRange::Single(date) => date,
        DateRange::Range(start_date, end_date) =>
            format!("{} - {}", start_date, end_date),
    };

    view! {
        <span>{move || date_text.clone()}</span>
    }
}

fn render_status(status: DateStatus) -> impl IntoView {
    let status_text = match status {
        DateStatus::Upcoming => "Upcoming",
        DateStatus::Open => "Open",
        DateStatus::Deadline => "Deadline",
        DateStatus::Closed => "Closed",
    };

    let background_class = match status {
        DateStatus::Upcoming => "bg-yellow-100 text-yellow-800 border-yellow-100",
        DateStatus::Open => "bg-green-100 text-green-800 border-green-100",
        DateStatus::Deadline | DateStatus::Closed => "bg-red-100 text-red-800 border-red-100",
    };

    let classes =
        format!("inline-flex items-center justify-center rounded-md border px-2 py-0.5 text-xs
            font-medium w-fit whitespace-nowrap shrink-0 {}",
            background_class
        );

    view! {
        <span class=classes>
            {move || status_text}
        </span>
    }
}

/// # Date Component
/// A single date, for use in a [`DateList`] component.
///
/// Example usage:
/// ```
/// let date_info = DateInfo {
///     title: "Example Title".to_string(),
///     date: DateRange::Single("January 1st, 1990".to_string()),
///     description: "A description of the date.".to_string(),
///     status: DateStatus::Upcoming
/// }
///
/// view! {
///     <Date
///         info=date_info
///     />
/// }
/// ```
#[component]
fn Date(
    /// Struct of title, date, and description
    #[prop(into)] info: Signal<DateInfo>,
) -> impl IntoView {
    view! {
        <div class="flex items-start gap-3 rounded-lg border-grey-300 p-3
                   w-full text-left shadow-lg">
            // <div class="flex-shrink-0 mt-0.5">
            //     <img src={icon} class="h-4 w-4" alt="icon"/>
            // </div>
            <div class="flex-1 min-w-0">
                <div class="flex items-start justify-between">
                    <div class="flex-1">
                        <h3 class="font-bold text-md text-base">
                            {info.get().title}
                        </h3>
                        <h3 class="text-md text-base mt-1">
                            {move || render_dates(info.get().date)}
                        </h3>
                        <p class="text-sm text-gray-600">
                            {info.get().description}
                        </p>
                    </div>
                    <div class="flex-shrink-0 ml-2">
                        {move || render_status(info.get().status)}
                    </div>
                </div>
            </div>
        </div>
    }
}

///# Dates List Component
/// Displays a list of important dates in a column, with badges that draw attention to dates that
/// are more important.
///
/// Example usage:
/// ```
/// let dates = vec![
///     DateInfo {
///         title: "Example Title".to_string(),
///         date: DateRange::Single("January 1st, 1990".to_string()),
///         description: "A description of the date.".to_string(),
///         status: DateStatus::Upcoming
///     }
/// ]
///
/// view! {
///     <DateList dates=dates />
/// }
/// ```
#[component]
pub fn DateList(
    #[prop(into)] dates: Vec<DateInfo>,
) -> impl IntoView {
    view! {
        {move || {
            dates.iter()
                .map(|info| {
                    view! {
                        <Date
                            info=info.clone()
                        />
                    }
                }).collect_view()
        }}
    }
}
