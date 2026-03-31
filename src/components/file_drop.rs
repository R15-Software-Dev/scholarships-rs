use crate::pages::api::files::{DeleteFile, upload_file};
use leptos::html::{Input, Label};
use leptos::logging::{debug_error, debug_log};
use leptos::prelude::*;
use leptos::web_sys::{File, FormData};
use leptos_icons::Icon;
use leptos_oidc::AuthSignal;
use leptos_use::{UseDropZoneOptions, UseDropZoneReturn, use_drop_zone_with_options};

#[component]
pub fn FileDrop(
    /// The name of this input. Used in file organization.
    #[prop(into)]
    name: String,
    /// The ID of the form containing this input. Used in file organization.
    #[prop(into)]
    form_id: String,
    /// The types that a file is allowed to be. Must be specified as the desired file extension (i.e.
    /// `.pdf`, `.txt`, etc.)
    #[prop(optional)]
    allowed_types: Vec<String>,
    /// A [`Resource`] that gives the currently uploaded files for this input.
    #[prop()]
    existing_files: Resource<Result<Vec<String>, ServerFnError>>,
) -> impl IntoView {
    let zone_ref = NodeRef::<Label>::new();
    let input_ref = NodeRef::<Input>::new();

    let file_name_list = RwSignal::new(Vec::new());
    let hovering = RwSignal::new(false);
    let error_msg = RwSignal::new(String::new());
    let name = StoredValue::new(name);
    let form_id = StoredValue::new(form_id);
    let auth = expect_context::<AuthSignal>();

    let upload_action = Action::new(|form_data: &FormData| {
        let form_data = form_data.clone();
        async move { upload_file(form_data.into()).await }
    });

    let delete_action = ServerAction::<DeleteFile>::new();

    let uploading = upload_action.pending();
    let deleting = delete_action.pending();

    let verify_file_type = move |file_name: String| {
        if !allowed_types.is_empty() {
            let verified = allowed_types
                .iter()
                .any(|file_ext| file_name.ends_with(file_ext));
            if !verified {
                debug_log!("Blocking file with name {file_name}");
            } else {
                debug_log!("Allowing file with name {file_name}");
            }
            verified
        } else {
            true
        }
    };

    let upload_files = Callback::new(move |files: Vec<File>| {
        let form_data = FormData::new().unwrap();

        files.iter().for_each(|file| {
            form_data
                .append_with_blob_and_filename(&name.get_value(), file, &file.name())
                .unwrap();
        });

        form_data
            .append_with_str("form_id", form_id.get_value().as_str())
            .unwrap();

        let token = auth
            .try_with(|a| {
                a.authenticated()
                    .map(|authenticated| authenticated.access_token())
            })
            .flatten();

        if let Some(access_token) = token {
            form_data
                .append_with_str("access_token", access_token.as_str())
                .unwrap()
        }

        // Upload file to server.
        upload_action.dispatch(form_data);
    });

    let UseDropZoneReturn {
        is_over_drop_zone, ..
    } = use_drop_zone_with_options(
        zone_ref,
        UseDropZoneOptions::default()
            .on_over(move |_| hovering.set(true))
            .on_drop(move |ev| {
                debug_log!("Received dropped file. List has length {}", ev.files.len());
                ev.files
                    .iter()
                    .for_each(|file| upload_files.run(vec![file.to_owned()]));
            }),
    );

    let on_input_change = move |_| {
        debug_log!("Running on:change");

        // This runs when users select a file by clicking on the element. We aren't going
        // to link the file list with this input except to update it with new information.
        let files = input_ref
            .get()
            .and_then(|input| input.files())
            .map(|list| {
                (0..list.length())
                    .filter_map(|i| list.item(i))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let verified_files = files
            .iter()
            .filter_map(|file| verify_file_type(file.name().to_string()).then(|| file.to_owned()))
            .collect::<Vec<File>>();

        // input_ref.try_update(|input| {
        //     if let Some(input) = input {
        //         input.set_files(None);
        //     }
        // });

        if !verified_files.is_empty() {
            upload_files.run(verified_files);
        }
    };

    let on_click_file_delete = move |file_name: String| {
        let access_token = auth
            .try_with(|a| a.authenticated().map(|a| a.access_token()))
            .flatten();

        delete_action.dispatch(DeleteFile {
            form_id: form_id.get_value(),
            input_name: name.get_value(),
            access_token: access_token.unwrap_or_default(),
            file_name,
        });
    };

    // Add file names to the list when upload_action is successful.
    Effect::new(move || {
        if let Some(Ok(uploaded_file_name)) = upload_action.value().get() {
            file_name_list.update(|list| {
                list.iter()
                    .find(|v| **v == uploaded_file_name)
                    .is_none()
                    .then(|| list.push(uploaded_file_name));
            });
        }
    });

    // Remove file name from the list when delete_action is successful.
    Effect::new(move || {
        if let Some(Ok(deleted_file_name)) = delete_action.value().get() {
            file_name_list.update(|list| {
                list.retain(|f| *f != deleted_file_name);
            });
        }
    });

    view! {
        {move || {
            existing_files
                .get()
                .map(|result| {
                    match result {
                        Ok(files) => {
                            error_msg.set("".to_string());
                            file_name_list.set(files);
                        }
                        Err(err) => {
                            let msg = err.to_string();
                            debug_error!("Error while getting files: {}", msg);
                            error_msg.set(msg);
                        }
                    }
                });
        }}
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
            <input node_ref=input_ref type="file" class="hidden" on:change=on_input_change />
        </label>
        <Show when=move || !error_msg.get().is_empty()>
            <div class="flex-1">{error_msg}</div>
        </Show>
        <For
            each=move || file_name_list.get()
            key=|file| file.clone()
            children=move |file| {
                let file = StoredValue::new(file);
                let on_click = move |_| {
                    on_click_file_delete(file.get_value());
                };
                view! {
                    <div class="m-1.5 flex flex-row relative items-center">
                        <div class="flex-1">{file.get_value()}</div>
                        <div
                            class="text-gray-400 hover:text-red-700 right-0 cursor-pointer"
                            on:click=on_click
                        >
                            <Icon icon=icondata::FaTrashCanRegular />
                        </div>
                    </div>
                }
            }
        />
    }
}
