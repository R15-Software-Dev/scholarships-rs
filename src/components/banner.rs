use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

/// # Banner Component
///
/// This component displays a red banner at the top of the page with a title and a logo.
/// Clicking the logo will redirect a user to specified page, preferably a home page.
///
/// Example usage:
/// ```
///  <Banner
///     title="Test Page"
///     logo="medal.png"
///     path="/"
///  />
/// ```
#[component]
pub fn Banner(
    /// The title of the banner.
    #[prop(into)]
    title: String,
    /// The file path or URL of a logo displayed at the top left of the banner.
    #[prop(into)]
    logo: String,
    /// The route that the navigates user to a page (home page).
    #[prop(into, default = String::from("/"))]
    path: String,
) -> impl IntoView {
    let navigate = use_navigate();

    let on_click = move |_| navigate(&path, Default::default());

    view! {
        <div class="mx-auto">
          <div class="bg-red-900 shadow-lg shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3),0_4px_6px_-1px_rgba(127,29,29,0.3),0_2px_4px_-1px_rgba(127,29,29,0.2)]">
            <div class="flex items-center gap-4 p-6">
              <div class="flex-shrink-0">
                <img src={logo.clone()} class="h-10 w-10 cursor-pointer" alt="icon" on:click=on_click/>
              </div>
              <div class="flex-1">
                <h2 class="text-white text-lg font-bold">{title.clone()}</h2>
              </div>
            </div>
          </div>
        </div>

    }
}
