use leptos::ev::{DragEvent, Event};
use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{Blob, BlobPropertyBag, File, HtmlInputElement, Url, window};

#[derive(Debug, Clone, PartialEq)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub file_type: String,
    pub status: FileStatus,
    pub progress: f64,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileStatus {
    #[default]
    Pending,
    Uploading,
    Completed,
    Error,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UploadProgress {
    pub file_id: String,
    pub progress: f64,
    pub bytes_uploaded: u64,
    pub total_bytes: u64,
}

fn handle_files(
    list: web_sys::FileList,
    allowed: Option<Vec<String>>,
    on_change: Callback<FileInfo>,
) {
    for i in 0..list.length() {
        if let Some(file) = list.get(i) {
            let name = file.name();
            let mime = file.type_();
            let size = file.size() as u64;

            // File extension restriction
            if let Some(allowed_types) = &allowed {
                let ok = allowed_types.iter().any(|ext| name.ends_with(ext));
                if !ok {
                    logging::log!("Rejected file: {}", name);
                    continue;
                }
            }

            let info = FileInfo {
                id: uuid::Uuid::new_v4().to_string(),
                name,
                size,
                file_type: mime,
                status: FileStatus::Pending,
                progress: 0.0,
                error_message: None,
            };

            logging::log!("Accepted file: {:?}", info);
            on_change.run(info);
        }
    }
}

#[component]
pub fn FileUpload(
    #[prop()] file_types: Option<Vec<String>>,
    #[prop(into)] on_change: Callback<FileInfo>,
) -> impl IntoView {
    let file_data = RwSignal::new(None::<Vec<u8>>);

    let drag_over = move |ev: DragEvent| ev.prevent_default();

    let drop = {
        let file_types = file_types.clone();
        let on_change = on_change.clone();
        move |ev: DragEvent| {
            ev.prevent_default();
            if let Some(dt) = ev.data_transfer() {
                if let Some(list) = dt.files() {
                    handle_files(list, file_types.clone(), on_change.clone());
                }
            }
        }
    };

    let on_change = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = event_target(&ev);

        if let Some(file) = input.files().and_then(|fs| fs.get(0)) {
            // Read as bytes
            let fut = async move {
                let array_buf = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
                    .await
                    .unwrap();

                let bytes = js_sys::Uint8Array::new(&array_buf).to_vec();
                Some(bytes) // Returns file bytes
            };

            // Spawn a new thread that runs the move function
            spawn_local(async move {
                let bytes = fut.await;
                file_data.set(bytes); // Sets value of file_data signal across this singular component
            });
        }
    };

    view! {
        <div
            class="file-upload"
            on:dragover=drag_over
            on:drop=drop
        >
            <input type="file" on:change=on_change />
            <Show when=move || file_data.get().is_some()> // file_data.get() returns the Option<Vec<u8>> in the RwSignal
                <button on:click=move |_| {
                    if let Some(bytes) = file_data.get() {
                        open_in_new_tab(bytes, "application/octet-stream");
                    }
                }> "Open in New Tab" </button>
            </Show>
        </div>
    }
}

pub fn open_in_new_tab(bytes: Vec<u8>, mime: &str) {
    // Create a JS `Uint8Array`
    let uint8 = js_sys::Uint8Array::from(bytes.as_slice());

    // Create Blob
    let mut bag = BlobPropertyBag::new();
    bag.set_type(mime);

    let blob =
        Blob::new_with_u8_array_sequence_and_options(&js_sys::Array::of1(&uint8), &bag).unwrap();

    // Create object URL
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Open new tab
    window()
        .unwrap()
        .open_with_url_and_target(&url, "_blank")
        .unwrap();
}

// use crate::components::{merge_classes, generate_id};
// use leptos::{component, web_sys};
// use leptos::prelude::*;
// use leptos::callback::Callback;
// use leptos::children::Children;
//
// /// File Info structure
// #[derive(Debug, Clone, PartialEq)]
// pub struct FileInfo {
//     pub id: String,
//     pub name: String,
//     pub size: u64,
//     pub file_type: String,
//     pub status: FileStatus,
//     pub progress: f64,
//     pub error_message: Option<String>,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// pub enum FileStatus {
//     #[default]
//     Pending,
//     Uploading,
//     Completed,
//     Error,
//     Cancelled,
// }
//
// #[derive(Debug, Clone, PartialEq)]
// pub struct UploadProgress {
//     pub file_id: String,
//     pub progress: f64,
//     pub bytes_uploaded: u64,
//     pub total_bytes: u64,
// }
//
// fn is_file_allowed(
//     name: &str,
//     mime: &str,
//     allowed_types: &[String],
//     accept_str: &str,
// ) -> bool {
//     // If developer specified `file_types`, prefer that
//     if !allowed_types.is_empty() {
//         for pattern in allowed_types {
//             let pattern = pattern.trim();
//             if pattern.is_empty() {
//                 continue;
//             }
//
//             if pattern.starts_with('.') {
//                 // Treat as extension, e.g. ".pdf"
//                 if name.to_lowercase().ends_with(&pattern.to_lowercase()) {
//                     return true;
//                 }
//             } else {
//                 // Treat as MIME type, e.g. "application/pdf"
//                 if mime.eq_ignore_ascii_case(pattern) {
//                     return true;
//                 }
//             }
//         }
//         return false;
//     }
//
//     // Otherwise, fall back to the raw accept string, if provided
//     if !accept_str.trim().is_empty() {
//         for pattern in accept_str.split(',') {
//             let pattern = pattern.trim();
//             if pattern.is_empty() {
//                 continue;
//             }
//
//             if pattern.starts_with('.') {
//                 // Extension
//                 if name.to_lowercase().ends_with(&pattern.to_lowercase()) {
//                     return true;
//                 }
//             } else {
//                 // MIME type
//                 if mime.eq_ignore_ascii_case(pattern) {
//                     return true;
//                 }
//             }
//         }
//         return false;
//     }
//
//     // If no restrictions specified, allow everything
//     true
// }
//
// #[component]
// pub fn FileUpload(
//     #[prop(optional)] class: Option<String>,
//     #[prop(optional)] style: Option<String>,
//     #[prop(optional)] children: Option<Children>,
//     #[prop(optional)] multiple: Option<bool>, // Whether multiple files can be selected
//     #[prop(optional)] accept: Option<String>, // File type prop
//     #[prop(optional)] file_types: Option<Vec<String>>,
//     #[prop(optional)] max_size: Option<u64>,
//     #[prop(optional)] max_files: Option<usize>,
//     #[prop(optional)] disabled: Option<bool>,
//     #[prop(optional)] drag_drop_enabled: Option<bool>, // Enable drag & drop
//     #[prop(optional)] on_files_select: Option<Callback<Vec<FileInfo>>>,
//     #[prop(optional)] on_upload_progress: Option<Callback<UploadProgress>>,
//     #[prop(optional)] on_upload_complete: Option<Callback<Vec<FileInfo>>>,
//     #[prop(optional)] on_upload_error: Option<Callback<String>>,
//
// ) -> impl IntoView {
//     let _multiple = multiple.unwrap_or(false); // Only allow single file
//
//     // Build accept string from either explicit `accept` or `file_types`
//     let file_types = file_types.unwrap_or_default();
//     let _accept = accept.unwrap_or_else(|| {
//         if file_types.is_empty() {
//             String::new()
//         } else {
//             file_types.join(",")
//         }
//     });
//
//     let _max_size = max_size.unwrap_or(10 * 1024 * 1024); // 10MB default
//     let _max_files = max_files.unwrap_or(1); // Defaults to 1
//     let disabled = disabled.unwrap_or(false);
//     let drag_drop_enabled = drag_drop_enabled.unwrap_or(true);
//
//     let class = merge_classes(vec![
//         "file-upload",
//         if drag_drop_enabled {
//             "drag-drop-enabled"
//         } else {
//             "drag-drop-disabled"
//         },
//         "border-2 border-dashed m-1.5 p-1.5 mt-0 rounded-md bg-transparent \
//          relative flex-1 transition-all duration-150 flex items-center justify-center",
//         // enabled vs disabled state
//         if disabled {
//             "border-gray-600 pointer-events-none bg-gray-600/30"
//         } else {
//             "border-red-700 hover:border-red-500"
//         },
//         // Allow consumer to add/override classes
//         class.as_deref().unwrap_or(""),
//     ]);
//
//     // Clone values into the closure
//     let allowed_types_for_drop = file_types.clone();
//     let accept_str_for_drop = _accept.clone();
//     let on_files_select_for_drop = on_files_select.clone();
//     let on_upload_error_for_drop = on_upload_error.clone();
//
//     // Drag & drop handlers
//     let handle_drop = move |event: web_sys::DragEvent| {
//         if !disabled && drag_drop_enabled {
//             event.prevent_default();
//             // File handling logic would be implemented here
//         }
//     };
//
//
//     let handle_dragover = move |event: web_sys::DragEvent| {
//         if !disabled && drag_drop_enabled {
//             event.prevent_default();
//
//             if let Some(data_transfer) = event.data_transfer() {
//                 if let Some(file_list) = data_transfer.files() {
//                     let length = file_list.length();
//                     let mut accepted_files = Vec::new();
//
//                     // Log raw file count
//                     web_sys::console::log_1(
//                         &format!("Dropped {} file(s)", length).into()
//                     );
//
//                     for i in 0..length {
//                         if let Some(file) = file_list.item(i) {
//                             let name = file.name();
//                             let size = file.size() as u64;
//                             let mime = file.type_();
//
//                             // Log each dropped file
//                             web_sys::console::log_1(
//                                 &format!(
//                                     "Dropped file: name='{}', size={}, type='{}'",
//                                     name, size, mime
//                                 )
//                                     .into(),
//                             );
//
//                             // Optional: enforce max_files
//                             if accepted_files.len() >= _max_files {
//                                 if let Some(cb) = &on_upload_error_for_drop {
//                                     cb.run(format!(
//                                         "Maximum file count ({}) exceeded",
//                                         _max_files
//                                     ));
//                                 }
//                                 break;
//                             }
//
//                             // Optional: enforce max_size
//                             if size > _max_size {
//                                 if let Some(cb) = &on_upload_error_for_drop {
//                                     cb.run(format!(
//                                         "File '{}' is too large ({} bytes). Max allowed is {} bytes.",
//                                         name, size, _max_size
//                                     ));
//                                 }
//                                 continue;
//                             }
//
//                             // Check allowed type
//                             if !is_file_allowed(
//                                 &name,
//                                 &mime,
//                                 &allowed_types_for_drop,
//                                 &accept_str_for_drop,
//                             ) {
//                                 if let Some(cb) = &on_upload_error_for_drop {
//                                     cb.run(format!(
//                                         "File '{}' (type '{}') is not an allowed type.",
//                                         name, mime
//                                     ));
//                                 }
//                                 continue;
//                             }
//
//                             // Build FileInfo
//                             let info = FileInfo {
//                                 id: generate_id(),
//                                 name,
//                                 size,
//                                 file_type: mime,
//                                 status: FileStatus::Pending,
//                                 progress: 0.0,
//                                 error_message: None,
//                             };
//
//                             accepted_files.push(info);
//                         }
//                     }
//
//                     // Log final accepted files
//                     web_sys::console::log_1(
//                         &format!("Accepted {} file(s)", accepted_files.len()).into(),
//                     );
//
//                     // Invoke callback with accepted files
//                     if !accepted_files.is_empty() {
//                         if let Some(cb) = &on_files_select_for_drop {
//                             cb.run(accepted_files);
//                         }
//                     }
//                 }
//             }
//         }
//     };
//
//     view! {
//         <div
//             class=class
//             style=style
//             role="button"
//             aria-label="File upload area"
//             data-multiple=multiple
//             data-accept=_accept
//             data-max-size=_max_size
//             data-max-files=_max_files
//             on:drop=handle_drop
//             on:dragover=handle_dragover
//             tabindex="0"
//         >
//             // If children is Some, call it to render whatever children were passed
//             {children.map(|c| c())}
//         </div>
//     }
//
// }
