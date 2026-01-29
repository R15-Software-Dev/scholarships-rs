use leptos::prelude::*;
use leptos_animate::animate;
use leptos_animate::animations::classes::{In, Out};
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::use_location;
use crate::common::{SubTabInfo, TabInfo};

/// # Tab Sidebar Component
/// 
/// This component creates a list of tabs that correspond to the given `Vec<TabInfo>`.
/// There are a few things to note:
/// 
///  - This component defines an [`Outlet`], meaning that it is designed to display the route
///    information on its own. If you intend to use this on a page, the page should not also contain
///    an `Outlet` component unless it needs to render its own nested children.
///  - The "selected" appearance for the individual tabs work best when their paths are at the same
///    depth. For example:
/// ```
/// let good_list = vec! [
///     TabInfo {
///         ...,
///         path: "/default/path"
///     },
///     TabInfo {
///         ...,
///         path: "/good/path"
///     }
/// ];
/// 
/// let bad_list = vec! [
///     TabInfo {
///         ...,
///         path: "/default/path"
///     },
///     TabInfo {
///         ...,
///         path: "/bad/path/here"
///     }
/// ]
/// ```
/// 
/// Example usage in a view (assuming the good list from above):
/// ```
/// view! {
///     <TabSidebarList
///         tabs=good_list
///     />
///     // Remember, no Outlets!
/// }
/// ```
#[component]
pub fn TabSidebarList(
    #[prop(into)] tabs: Signal<Vec<TabInfo>>,
) -> impl IntoView {
    // Create a series of tabs based on a given list of names and paths.

    view! {
        <div class="flex flex-row h-full w-full fixed">
            <nav class="flex flex-col bg-red-900 w-64 h-full overflow-scroll">
                <For
                    each=move || tabs.get()
                    key=|info| info.text.clone()
                    let(TabInfo { text, path, sub_paths })
                >
                    <SidebarTab text=text path=path sub_paths=sub_paths />
                </For>
            </nav>

            <div class="flex flex-1 overflow-y-scroll">
                <Outlet />
            </div>
        </div>
    }
}

#[component]
fn SidebarTab(
    #[prop(into)] text: Signal<String>,
    #[prop(into)] path: Signal<String>,
    #[prop(into)] sub_paths: Signal<Vec<SubTabInfo>>,
) -> impl IntoView {
    let location = use_location();

    let selected = Memo::new(move |_| {
        let current_path = location.pathname.get();

        // There's extra logic here as well, but for now this works.
        current_path.starts_with(&path.get())
    });

    view! {
        <A href=move || path.get()>
            <div
                class="p-3 transition-all font-bold"
                class=(["bg-transparent", "text-white"], move || !selected.get())
                class=(["bg-white", "text-black"], move || selected.get())
            >
                {text}
            </div>
        </A>
        <For
            each=move || sub_paths.get()
            key=|info| info.text.clone()
            let(SubTabInfo { text, path })
        >
            <SidebarSubTab
                use:animate=(
                    In::default()
                    .source("h-0")
                        .target("h-auto")
                        .active("transition-all"),
                    Out::default()
                        .source("h-auto")
                        .target("h-0")
                        .active("transition-all")
                )
                text=text path=path visible=selected />
        </For>
    }
}

#[component]
fn SidebarSubTab(
    #[prop(into)] text: Signal<String>,
    #[prop(into)] path: Signal<String>,
    #[prop(into)] visible: Signal<bool>,
) -> impl IntoView {
    let selected = Memo::new(move |_| {
        visible.get()
    });
    
    view! {
        <div class="grid transition-all ease-in-out"
            style:grid-template-rows=move || if visible.get() { "1fr" } else { "0fr" }
            style:opacity=move || if visible.get() { "1" } else { "0" }
        >
            <div
                class="overflow-hidden p-3 pl-10 transition-all bg-white"
                class=(["font-bold"], move || selected.get())
            >
                {text}
            </div>
        </div>
    }
}
