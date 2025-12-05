use leptos::prelude::*;

/// # Date Component
///
///
/// Example usage:
/// ```

/// ```
#[component]
pub fn Date(
    /// The main label displayed on the button.
    #[prop(into)]
    title: String,
    /// The calendar date of the event
    #[prop(into)]
    date: String,
    /// A brief explanatory text that describes the destination page.
    #[prop(into)]
    description: String,
    /// The file path or URL of the icon displayed at the top of the button.
    #[prop(into)]
    icon: String,
    // The status of the event (open, upcoming, deadline...)
    //#[prop(into)] status: String,
) -> impl IntoView {
    view! {
        <div class="dashboard-button flex items-start gap-3 rounded-lg border-grey-300 p-3
                   hover:bg-gray-100 transition w-full text-left
                   shadow-[inset_0_0_6px_rgba(0,0,0,0.12)]">
            <div class="flex-shrink-0 mt-0.5">
                <img src={icon.clone()} class="h-4 w-4" alt="icon"/>
            </div>
            <div class="flex flex-col">
                <h3 class="font-semibold text-md text-base">
                    {title.clone()}
                </h3>
                <h3 class="font-bold text-md text-base mt-1">
                    {date.clone()}
                </h3>
                <p class="text-sm text-gray-600 mt-1">
                    {description.clone()}
                </p>
            </div>
        </div>
    }
}
