use leptos::prelude::*;
use leptos_router::components::{Outlet, A};
use leptos_router::hooks::use_location;
use crate::common::TabInfo;

#[component]
fn SidebarTab(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] path: Signal<String>
) -> impl IntoView {
    let location = use_location();

    let selected = Memo::new(move |_| {
        let current_path = location.pathname.get();

        // There's extra logic here as well, but for now this works.
        current_path.starts_with(&path.get())
    });

    view! {
        <A href={move || path.get()}>
            <div class="p-3 transition-all font-bold"
                class=(["bg-transparent", "text-white"], move || !selected.get())
                class=(["bg-white", "text-black"], move || selected.get())>
                {name}
            </div>
        </A>
    }
}

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
                    key=|info| info.name.clone()
                    let(TabInfo {name, path})
                >
                    <SidebarTab
                        name=name
                        path=path
                    />
                </For>
            </nav>

            <div class="flex flex-1">
                <Outlet />
            </div>
        </div>
    }
}
