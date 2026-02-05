use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_icons::Icon;

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
    logo: Signal<String>,
    /// The route that the navigates user to a page (home page).
    #[prop(into, optional)]
    path: Signal<String>,
) -> impl IntoView {
    let navigate = use_navigate();

    let show_back = Signal::derive(move || !path.get().is_empty());
    
    let on_click = move |_| {
        let path = path.get();
        if !path.is_empty() {
            navigate(&path, Default::default());
        }
    };

    view! {
        <div class="w-screen bg-red-900 shadow-lg shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3),0_4px_6px_-1px_rgba(127,29,29,0.3),0_2px_4px_-1px_rgba(127,29,29,0.2)]">
            <div class="flex flex-row items-center gap-4 p-6 pb-2">
                <div class="flex-shrink-0">
                    <img
                        src=logo
                        class="h-10 w-10 cursor-pointer"
                        alt="icon"
                        on:click=on_click
                    />
                </div>
                <div class="flex-1">
                    <h2 class="text-white text-2xl font-bold">{title.clone()}</h2>
                </div>
            </div>
            <div
                class="flex flex-row items-center gap-4"
                class=("p-2", move || !show_back.get())
            >
                <Show when=move || show_back.get()>
                    <a
                        href=path
                        class="flex flex-row items-center gap-2 m-2 ml-14 text-gray-200 hover:underline"
                    >
                        <Icon icon=icondata::FaArrowLeftSolid />
                        <span>"Back to Dashboard"</span>
                    </a>
                </Show>
            </div>
        </div>
    }
}
