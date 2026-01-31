use leptos::logging::log;
use leptos::prelude::*;
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
    #[prop(into)] base_path: Signal<String>,
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
                    <SidebarTab base_path=base_path text=text path=path sub_paths=sub_paths />
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
    #[prop(into)] base_path: Signal<String>,
    #[prop(into)] text: Signal<String>,
    #[prop(into)] path: Signal<String>,
    #[prop(into)] sub_paths: Signal<Vec<SubTabInfo>>,
) -> impl IntoView {
    let location = use_location();

    let full_path = Memo::new(move |_| {
        format!("/{}/{}", base_path.get(), path.get())
    });

    let selected = Memo::new(move |_| {
        let current_path = location.pathname.get();

        // There's extra logic here as well, but for now this works.
        current_path.starts_with(&full_path.get())
    });

    view! {
        <div 
            class="transition-all duration-300"
            class=(["bg-transparent", "text-white"], move || !selected.get())
            class=(["bg-white", "text-black"], move || selected.get())
        >
            <A href=move || path.get()>
                <div
                    class="p-3 transition-bg font-bold"
                >
                    {text}
                </div>
            </A>
            <div
                class="grid transition-all ease-in-out"
                style:grid-template-rows=move || if selected.get() { "1fr" } else { "0fr" }
            >
                <div class="overflow-hidden min-h-0">
                    <For
                        each=move || sub_paths.get()
                        key=|info| info.text.clone()
                        let(SubTabInfo { text, path })
                    >
                        <SidebarSubTab parent_path=full_path text=text path=path />
                    </For>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SidebarSubTab(
    #[prop(into)] parent_path: Signal<String>,
    #[prop(into)] text: Signal<String>,
    #[prop(into)] path: Signal<String>,
) -> impl IntoView {
    let full_path = Memo::new(move |_| {
        format!("{}/{}", parent_path.get(), path.get())
    });

    view! {
        <A href=move || full_path.get() {..} class="block p-3 pl-10 transition-all aria-[current]:font-bold">
            {text}
        </A>
    }
}
