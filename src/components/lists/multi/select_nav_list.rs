use leptos::logging::log;
use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::utils;

#[derive(Debug, Clone)]
pub struct NavListInfo {
    pub text: Vec<String>,
    pub id: String,
    pub slug: String,
}

#[component]
fn NavListItem(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] text: Signal<Vec<String>>,
    #[prop(into)] slug: Signal<String>,
    #[prop(into)] selected: Signal<bool>,
    #[prop(into)] on_change: Callback<()>,
) -> impl IntoView {
    let linked_info = Signal::derive(move ||
        text.get()
            .get(0)
            .cloned()
            .unwrap_or_default()
    );
    let other_info = Signal::derive(move ||
        text.get()
            .iter()
            .cloned()
            .skip(1)
            .collect::<Vec<String>>()
    );

    view! {
        <div
            class="flex flex-row gap-2 p-1 rounded-lg"
            class=(["bg-yellow-100"], move || selected.get())
        >
            <NavListCheckbox
                name=name
                value=value
                on_change=move || on_change.run(())
                selected=selected
            />
            <A
                href=move || slug.get()
                {..}
                class="flex flex-1 items-center underline hover:text-blue-400"
            >
                {linked_info}
            </A>
            <For each=move || other_info.get() key=|info| info.clone() let(info)>
                <p class="flex flex-1 items-center">{info}</p>
            </For>
        </div>
    }
}

#[component]
fn NavListCheckbox(
    #[prop(into)] selected: Signal<bool>,
    #[prop(into)] on_change: Callback<()>,
    #[prop(into)] name: Signal<String>,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    let id = Memo::new(move |_| utils::create_unique_id(&name.get(), &value.get()));

    view! {
        <label for=id class="flex items-center">
            <input
                class="hidden relative peer shrink-0"
                type="checkbox"
                name=name
                id=id
                prop:checked=selected
                on:change=move |_| on_change.run(())
            />
            <div class="relative inline-block h-5 w-5 m-2 rounded-sm border border-gray-300 bg-white
            transition-bg duration-150 peer-checked:bg-red-700 peer-checked:border-red-700
            [&>span]:opacity-0 peer-checked:[&>span]:opacity-100
            peer-disabled:bg-gray-600/33 peer-disabled:border-gray-600
            peer-disabled:peer-checked:bg-gray-600">
                // This span is the check icon. It is centered within the div and has a border on the bottom
                // and right sides, which is then rotated to appear like a checkmark.
                <span class="absolute left-1/2 top-1/2 block h-3 w-1.5 -translate-x-1/2 -translate-y-1/2
                rotate-45 transform border-b-3 border-r-3
                border-white transition-opacity duration-150"></span>
            </div>
        </label>
    }
}

/// # Selectable Nav-List Component
/// 
/// This component takes a `Vec<NavListInfo>` to show a list. The list contains links that append
/// the `NavListItem::slug` to the current path, and each item is selectable. The list of selected
/// items is usable in this component's parent.
#[component]
pub fn SelectableNavList(
    #[prop(into)] selected: RwSignal<Vec<String>>,
    #[prop(into)] items: Signal<Vec<NavListInfo>>,
    #[prop(into)] headers: Signal<Vec<String>>,
    #[prop(into)] name: Signal<String>,
) -> impl IntoView {
    let all_selected = RwSignal::new(false);
    let all_selected_change = move || {
        if all_selected.get() {
            log!("Unchecking all values.");
            all_selected.set(false);
            selected.set(Vec::new());
        } else {
            log!("Checking all values.");
            all_selected.set(true);
            selected.set(
                items
                    .get()
                    .iter()
                    .map(|info| info.id.clone())
                    .collect::<Vec<String>>()
            );
        }
    };
    
    view! {
        <div class="flex flex-1 flex-col m-3">
            // Header block - contains evenly spaced headers and the "select all" checkbox.
            // Include a checkbox here that's got the same style as the items. This maintains spacing.
            // All headers *must* be evenly spaced, otherwise the list starts getting confusing.
            <div class="flex flex-row bg-red-800 rounded-lg p-1 gap-2">
                <NavListCheckbox
                    selected=all_selected
                    name=name
                    value="select-all"
                    on_change=all_selected_change
                />
                <For each=move || headers.get() key=|header| header.clone() let(header_text)>
                    <div class="flex flex-1 text-white font-bold items-center">{header_text}</div>
                </For>
            </div>
            // Items block - contains constructed items. These are selectable.
            <For
                each=move || items.get()
                key=|item| item.id.clone()
                children=move |entry| {
                    let id = Signal::derive(move || entry.id.clone());
                    let text = Signal::derive(move || entry.text.clone());
                    let selected_sig = Signal::derive(move || selected.get().contains(&id.get()));
                    let on_change = move || {
                        selected
                            .update(|list| {
                                let id = id.get();
                                if list.contains(&id) {
                                    log!("Found value {}, removing.", id);
                                    list.retain(|s| s != &id);
                                } else {
                                    log!("Adding value {} to the list.", id);
                                    list.push(id);
                                }
                            });
                        all_selected.set(false);
                    };

                    view! {
                        <NavListItem
                            name=name
                            text=text
                            value=id.clone()
                            slug=entry.slug
                            selected=selected_sig
                            on_change=on_change
                        />
                    }
                }
            />
        </div>
    }
}
