use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// #Dashboard Button Component
///
/// This component displays a large panel-like button that redirects a user to a specified page.
/// It's best used on a user dashboard, where multiple instances of the button are present.
/// It has a title, description, and icon
///
/// Example usage:
/// ```
///     <div class="p-6 space-y-4 max-w-1/3">
///         <DashboardButton
///             title="Home Page"
///             description="Navigate to Home Page"
///             icon="square-user.svg"
///             path="/"
///         />
///     </div>
/// ```
#[component]
pub fn DashboardButton(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] icon: String,
    #[prop(into)] path: String,
) -> impl IntoView {
    let navigate = use_navigate();

    let on_click = move |_| navigate(&path, Default::default());

    view! {
        <button
            type="button"
            class="dashboard-button flex items-start gap-3 rounded-lg border-grey-300 p-6
                   hover:bg-gray-100 transition cursor-pointer w-full text-left
                   shadow-[inset_0_0_6px_rgba(0,0,0,0.12)]"
            on:click=on_click>

            <div class="flex flex-col">
                <img src={icon.clone()} class="h-8 w-8" alt="icon"/>
                <h3 class="font-semibold text-base">
                    {title.clone()}
                </h3>

                <p class="text-sm text-gray-600 pt-6">
                    {description.clone()}
                </p>

            </div>
        </button>
    }
}
