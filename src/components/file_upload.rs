use crate::common::{FileInfo, UploadStatus};
use base64::Engine;
use gloo_timers::callback::Timeout;
use js_sys::Uint8Array;
use leptos::ev::{DragEvent, Event};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use web_sys::{File, FileList, HtmlAnchorElement, HtmlInputElement};

/// # File Upload Component
///
/// This component renders a drag-and-drop file upload area with optional file type validation,
/// single or multi-file support, visual feedback on success and error, and a list of uploaded files.
///
/// It supports:
/// - Drag-and-drop and click-to-browse file selection
/// - Optional restriction to specific file extensions
/// - Optional single-file or multi-file mode
/// - Automatic validation and error messaging
/// - A visual success/error flash on upload
/// - Display of uploaded file names with a "Clear all" action
/// - Emitting structured file metadata to the parent component
///
/// The parent component receives updates whenever the uploaded file list changes.
///
/// ## Example usage:
/// ```
/// use leptos::prelude::*;
/// use crate::components::file_upload::{FileUpload, FileInfo};
///
/// #[component]
/// pub fn Example() -> impl IntoView {
///     let files = RwSignal::new(Vec::<FileInfo>::new());
///
///     let on_change = Callback::new(move |new_files: Vec<FileInfo>| {
///         files.set(new_files);
///     });
///
///     view! {
///         <div class="p-6 space-y-4 max-w-lg">
///             <FileUpload
///                 label="Upload Enrollment Data"
///                 description="Upload one or more CSV files exported from the registrar system."
///                 file_types=Some(vec![".csv".into()])
///                 limit_upload=false
///                 on_change=on_change
///             />
///         </div>
///     }
/// }
/// ```

// Normalize file extensions to lowercase and ensure they start with '.'.
fn normalize_extensions(exts: &[String]) -> Vec<String> {
    exts.iter()
        .map(|e| {
            let trimmed = e.trim().to_lowercase();
            if trimmed.starts_with('.') {
                trimmed
            } else {
                format!(".{trimmed}")
            }
        })
        .collect()
}

/// Check if a file matches the allowed extension list.
fn file_is_valid(file: &File, normalized_exts: &[String]) -> bool {
    if normalized_exts.is_empty() {
        return true;
    }

    let name = file.name().to_lowercase();
    normalized_exts.iter().any(|ext| name.ends_with(ext))
}

/// Convert a FileList into a Rust Vec<File>.
fn filelist_to_vec(list: &FileList) -> Vec<File> {
    let mut out = Vec::new();
    for i in 0..list.length() {
        if let Some(f) = list.get(i) {
            out.push(f);
        }
    }
    out
}

// Build the public FileInfo structure from a browser File object.
fn build_file_info(file: &File) -> FileInfo {
    FileInfo {
        id: uuid::Uuid::new_v4().to_string(),
        name: file.name(),
        size: file.size() as u64,
        file_type: file.type_(),
        progress: 0.0,
        error_message: None,
    }
}

// Open the uploaded file in a new browser tab using a data URL.
pub fn open_in_new_tab(bytes: Vec<u8>, mime: &str) {
    let base64 = base64::engine::general_purpose::STANDARD.encode(bytes);
    let data_url = format!("data:{};base64,{}", mime, base64);

    let link = document()
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    link.set_href(&data_url);
    link.set_target("_blank");
    link.click();
}

#[component]
pub fn FileUpload(
    /// Text displayed above the upload area
    #[prop(into)]
    label: String,
    /// Optional helper text displayed below the label
    #[prop(optional, into)]
    description: Option<String>,
    /// Optional list of allowed file extensions (e.g. `[".csv", ".xlsx"]`)
    #[prop()]
    file_types: Option<Vec<String>>,
    /// If true, only one file may be uploaded at a time
    #[prop(optional)]
    limit_upload: bool,
    /// Callback triggered whenever the file list changes
    #[prop(into)]
    on_change: Callback<Vec<FileInfo>>,
) -> impl IntoView {
    let files = RwSignal::new(Vec::<FileInfo>::new());

    let last_file_bytes = RwSignal::new(None::<Vec<u8>>);
    let last_file_mime = RwSignal::new(None::<String>);

    let is_dragging = RwSignal::new(false);
    let upload_status = RwSignal::new(UploadStatus::Idle);
    let error_message = RwSignal::new(None::<String>);

    let desc_sig = RwSignal::new(description);
    let exts_sig = RwSignal::new(normalize_extensions(
        &file_types.clone().unwrap_or_default(),
    ));

    let accept_attr = move || exts_sig.get().join(",");

    let accepted_text = move || {
        let exts = exts_sig.get();
        if exts.is_empty() {
            "any".to_string()
        } else {
            exts.join(", ")
        }
    };

    let auto_clear_visual_state = {
        let upload_status = upload_status;
        let is_dragging = is_dragging;
        move || {
            Timeout::new(1000, move || {
                upload_status.set(UploadStatus::Idle);
                is_dragging.set(false);
            })
            .forget();
        }
    };

    let add_files: Callback<Vec<File>> = {
        let files_sig = files;
        let on_change = on_change.clone();
        let error_message = error_message;
        let upload_status = upload_status;
        let auto_clear_visual_state = auto_clear_visual_state;
        let last_file_bytes = last_file_bytes;
        let last_file_mime = last_file_mime;
        let exts_sig = exts_sig;
        let limit_upload = limit_upload;

        Callback::new(move |incoming: Vec<File>| {
            let exts = exts_sig.get();
            let enforce_types = !exts.is_empty();

            if limit_upload && incoming.len() > 1 {
                upload_status.set(UploadStatus::Error);
                error_message.set(Some("Only one file may be uploaded at a time.".to_string()));
                auto_clear_visual_state();
                return;
            }

            let (mut valid, mut invalid): (Vec<File>, Vec<File>) = (Vec::new(), Vec::new());

            for f in incoming {
                if !enforce_types || file_is_valid(&f, &exts) {
                    valid.push(f);
                } else {
                    invalid.push(f);
                }
            }

            if !valid.is_empty() {
                upload_status.set(UploadStatus::Success);
                error_message.set(None);
                auto_clear_visual_state();
            }

            if valid.is_empty() && !invalid.is_empty() {
                let invalid_names = invalid
                    .iter()
                    .map(|f| f.name())
                    .collect::<Vec<_>>()
                    .join(", ");
                let allowed = if enforce_types {
                    exts.join(", ")
                } else {
                    "any".to_string()
                };

                upload_status.set(UploadStatus::Error);
                error_message.set(Some(format!(
                    "Invalid file type: {invalid_names}. Only {allowed} files are allowed."
                )));
                auto_clear_visual_state();
                return;
            }

            if limit_upload {
                let Some(single) = valid.into_iter().next() else {
                    return;
                };

                let info = build_file_info(&single);
                files_sig.set(vec![info.clone()]);
                on_change.run(vec![info]);

                let mime = single.type_();
                last_file_mime.set(Some(mime.clone()));
                spawn_local(async move {
                    if let Ok(array_buf) =
                        wasm_bindgen_futures::JsFuture::from(single.array_buffer()).await
                    {
                        let bytes = Uint8Array::new(&array_buf).to_vec();
                        last_file_bytes.set(Some(bytes));
                    }
                });

                return;
            }

            let mut existing = std::collections::HashSet::<String>::new();
            for f in files_sig.get().iter() {
                existing.insert(f.name.clone());
            }

            let mut merged = files_sig.get();
            for f in valid {
                if !existing.contains(&f.name()) {
                    merged.push(build_file_info(&f));
                }
            }

            files_sig.set(merged.clone());
            on_change.run(merged);
        })
    };

    let on_drag_over = move |ev: DragEvent| {
        ev.prevent_default();
        is_dragging.set(true);
    };

    let on_drag_leave = move |_ev: DragEvent| {
        is_dragging.set(false);
    };

    let on_drop = {
        let add_files = add_files.clone();
        move |ev: DragEvent| {
            ev.prevent_default();
            is_dragging.set(false);

            if let Some(dt) = ev.data_transfer() {
                if let Some(list) = dt.files() {
                    let incoming = filelist_to_vec(&list);
                    if !incoming.is_empty() {
                        add_files.run(incoming);
                    }
                }
            }
        }
    };

    let on_change_input = {
        let add_files = add_files.clone();
        move |ev: Event| {
            let input: HtmlInputElement = event_target(&ev);

            if let Some(list) = input.files() {
                let incoming = filelist_to_vec(&list);
                if !incoming.is_empty() {
                    add_files.run(incoming);
                }
            }

            input.set_value("");
        }
    };

    let clear_all = {
        let files_sig = files;
        let on_change = on_change.clone();
        let upload_status = upload_status;
        let error_message = error_message;
        let last_file_bytes = last_file_bytes;
        let last_file_mime = last_file_mime;

        move |_| {
            files_sig.set(vec![]);
            on_change.run(vec![]);
            upload_status.set(UploadStatus::Idle);
            error_message.set(None);
            last_file_bytes.set(None);
            last_file_mime.set(None);
        }
    };

    let dropzone_classes = move || {
        let base =
            "border-2 border-dashed rounded-lg p-8 text-center cursor-pointer transition-colors";
        match upload_status.get() {
            UploadStatus::Success => format!("{base} border-green-500 bg-green-50"),
            UploadStatus::Error => format!("{base} border-red-500 bg-red-50"),
            UploadStatus::Idle => {
                if is_dragging.get() {
                    format!("{base} border-gray-300 bg-blue-50/20")
                } else {
                    format!("{base} border-gray-300 hover:border-red-700")
                }
            }
        }
    };

    let input_id = format!("file-input-{}", uuid::Uuid::new_v4());

    view! {
        <div class="space-y-2">
            <div class="flex items-center gap-2">
                <label class="text-sm font-medium">{label.clone()}</label>
            </div>

            <Show when=move || desc_sig.get().is_some()>
                <span class="text-xs text-gray-500">
                    {move || desc_sig.get().unwrap_or_default()}
                </span>
            </Show>

            <div
                class=dropzone_classes
                on:dragover=on_drag_over
                on:dragleave=on_drag_leave
                on:drop=on_drop
            >
                <input
                    id=input_id.clone()
                    type="file"
                    class="hidden"
                    prop:multiple=!limit_upload
                    prop:accept=accept_attr
                    on:change=on_change_input
                />

                <label for=input_id.clone() class="cursor-pointer block">
                    <div class="mx-auto mb-2 text-[#194678] text-3xl leading-none">"⬆"</div>

                    <p class="text-sm text-gray-600">
                        "Drag & drop or "
                        <span class="text-[#194678] underline">"browse files"</span>
                    </p>

                    <p class="text-xs text-gray-400 mt-1">
                        "Accepted: " {accepted_text} " "
                        {if limit_upload { "(single file)" } else { "(multiple allowed)" }}
                    </p>
                </label>
            </div>

            <Show when=move || !files.get().is_empty()>
                <div class="flex justify-between items-center">
                    <p class="text-xs text-gray-600">
                        "Uploaded: "
                        {move || files.get().into_iter().map(|f| f.name).collect::<Vec<_>>().join(", ")}
                    </p>

                    <button
                        type="button"
                        class="text-xs flex items-center gap-1 text-gray-500 hover:text-red-600"
                        on:click=clear_all
                    >
                        "✕ Clear all"
                    </button>
                </div>
            </Show>

            <Show when=move || error_message.get().is_some()>
                <p class="text-xs text-red-600 mt-1">
                    {move || error_message.get().unwrap_or_default()}
                </p>
            </Show>

            <Show when=move || last_file_bytes.get().is_some()>
                <div class="mt-1">
                    <button
                        type="button"
                        class="text-sm underline text-red-700 hover:text-red-900"
                        on:click=move |_| {
                            if let (Some(bytes), Some(mime)) = (last_file_bytes.get(), last_file_mime.get()) {
                                open_in_new_tab(bytes, &mime);
                            }
                        }
                    >
                        "Open in New Tab"
                    </button>
                </div>
            </Show>
        </div>
    }
}
