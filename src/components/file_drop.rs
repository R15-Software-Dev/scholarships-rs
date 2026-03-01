use leptos::html::{Input, Label};
use leptos::logging::debug_log;
use leptos::prelude::*;
use leptos::web_sys::{File, FormData};
use leptos_icons::Icon;
use leptos_oidc::AuthSignal;
use leptos_use::{UseDropZoneOptions, UseDropZoneReturn, use_drop_zone_with_options};
use crate::pages::api::files::upload_file;

#[component]
pub fn FileDrop(
    #[prop(into)] name: String,
    #[prop(into)] form_id: String,
) -> impl IntoView {
    let zone_ref = NodeRef::<Label>::new();
    let input_ref = NodeRef::<Input>::new();

    let file_name_list = RwSignal::new(Vec::new());
    let hovering = RwSignal::new(false);
    let name = StoredValue::new(name);
    let form_id = StoredValue::new(form_id);
    let auth = expect_context::<AuthSignal>();

    let upload_action = StoredValue::new(Action::new(|form_data: &FormData| {
        let form_data = form_data.clone();
        async move {
            upload_file(form_data.into()).await
        }
    }));

    let uploading = upload_action.get_value().pending();

    let upload_files = Callback::new(move |files: Vec<File>| {
        let form_data = FormData::new().unwrap();

        files.iter().for_each(|file| {
            let file_name = format!("{}_{}", name.get_value(), file.name());

            form_data.append_with_blob_and_filename(&name.get_value(), file, &file_name)
                .unwrap();
        });

        form_data.append_with_str("form_id", form_id.get_value().as_str()).unwrap();

        let token = auth.try_with(|a|
            a.authenticated().map(|authenticated| authenticated.access_token())
        ).flatten();

        if let Some(access_token) = token {
            form_data.append_with_str("access_token", access_token.as_str()).unwrap()
        }

        // Upload file to server.
        upload_action.get_value().dispatch(form_data);
    });

    let UseDropZoneReturn {
        is_over_drop_zone,
        ..
    } = use_drop_zone_with_options(
        zone_ref,
        UseDropZoneOptions::default()
            .on_over(move |_| hovering.set(true))
            .on_drop(move |ev| {
                let file = ev.files.first()
                    .unwrap();

                // Update file list
                file_name_list.update(|list| {
                    list.push(file.name());
                });

                upload_files.run(vec![file.to_owned()]);
            }),
    );

    let on_input_change = move |_| {
        debug_log!("Running on:change");

        // This runs when users select a file by clicking on the element. We aren't going
        // to link the file list with this input except to update it with new information.
        let files = input_ref.get()
            .and_then(|input| input.files())
            .map(|list| {
                (0..list.length())
                    .filter_map(|i| list.item(i))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        file_name_list.update({
            let files = files.clone();
            |list| {
                for file in files {
                    if !list.iter().any(|f| *f == file.name()) {
                        list.push(file.name());
                    }
                }
            }
        });

        upload_files.run(files);
    };

    view! {
        <label
            node_ref=zone_ref
            class="m-1.5 p-2.5 flex flex-col transition-color duration-200 rounded-lg border-2 items-center cursor-pointer"
            class=(["border-gray-400"], move || !is_over_drop_zone.get())
            class=(["border-blue-500"], move || is_over_drop_zone.get())
        >
            <div
                class="m-2 mb-4 transition-color duration-200 size-10"
                class=("text-gray-400", move || !is_over_drop_zone.get())
                class=("text-blue-500", move || is_over_drop_zone.get())
            >
                <Icon icon=icondata::FaFileCirclePlusSolid width="2.5rem" height="2.5rem" />
            </div>
            <div>"Drop a file here to upload"</div>
            <div class="text-sm text-gray-400">"Or click to select a file"</div>
            <input
                node_ref=input_ref
                type="file"
                class="hidden"
                on:change=on_input_change
            />
        </label>
        <For each=move || file_name_list.get() key=|file| file.clone() let:file>
            <div>"File name: "{file}</div>
        </For>
    }
}
