use leptos::html::Div;
use leptos::logging::debug_log;
use leptos::prelude::*;
use leptos_use::{UseDropZoneOptions, UseDropZoneReturn, use_drop_zone_with_options};

#[component]
pub fn FileDrop() -> impl IntoView {
    // I'm really not sure how to hydrate this, but we'll try our best to do it somehow.
    // We just need to know the file names and the size, not actually have the file itself.

    let zone_ref = NodeRef::<Div>::new();

    let file_list = RwSignal::new(Vec::new());

    // Files are just the current files that are being processed, not the full list of files that
    // have been dropped. So, if the user drops multiple files, it will be the list of those files,
    // but it will wipe the previous set of dropped files.
    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(
        zone_ref,
        UseDropZoneOptions::default()
            .on_over(|_| debug_log!("File is over the drop zone!"))
            .on_drop(move |mut ev| {
                file_list.update(|list| {
                    list.extend(ev.files);
                });
            }),
    );

    Effect::new(move || {
        let file_names = file_list
            .get()
            .iter()
            .map(|file| file.name())
            .collect::<Vec<String>>()
            .join(", ");

        debug_log!("Updated file list. Current file names: {}", file_names);
    });

    view! {
        <div
            node_ref=zone_ref
            class="size-64"
            class=(["bg-blue-500"], move || is_over_drop_zone.get())
        />
        <For each=move || file_list.get() key=|file| file.name() let:file>
            <div>"File name: "{file.name()}</div>
        </For>
    }
}
