use leptos::ev::{DragEvent, Event};
use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{Blob, BlobPropertyBag, File, HtmlInputElement, Url, window};

/// File metadata passed upward to the parent.
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

/// Current upload status of a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileStatus {
    #[default]
    Pending,
    Uploading,
    Completed,
    Error,
    Cancelled,
}

/// Upload progress (reserved for future use).
#[derive(Debug, Clone, PartialEq)]
pub struct UploadProgress {
    pub file_id: String,
    pub progress: f64,
    pub bytes_uploaded: u64,
    pub total_bytes: u64,
}

///
/// A shared function for handling both:
///  - drag & drop files
///  - file input selection
///
/// Performs:
///  - extension filtering (.pdf, .docx, etc.)
///  - updating displayed filename
///  - reading file bytes for preview
///  - notifying parent component via callback
///
fn process_file(
    file: File,
    allowed: &Option<Vec<String>>,
    on_change: &Callback<FileInfo>,
    file_data: RwSignal<Option<Vec<u8>>>,
    file_name: RwSignal<Option<String>>,
) {
    let name = file.name();
    let mime = file.type_();
    let size = file.size() as u64;

    // 🔒 Enforce extension whitelist (if provided)
    if let Some(allowed_types) = allowed {
        let ok = allowed_types.iter().any(|ext| name.ends_with(ext));
        if !ok {
            logging::log!("Rejected file due to extension mismatch: {}", name);
            file_data.set(None);
            file_name.set(None);
            return;
        }
    }

    // Store filename for display in the UI
    file_name.set(Some(name.clone()));

    // Read file into memory asynchronously for previewing
    let fut = async move {
        let array_buf = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
            .await
            .unwrap();

        let bytes = js_sys::Uint8Array::new(&array_buf).to_vec();
        Some(bytes)
    };

    // Run async file-reading in a Leptos task
    spawn_local(async move {
        let bytes = fut.await;
        file_data.set(bytes);
    });

    // Notify parent with a FileInfo struct
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

///
/// A fully restyled file upload component:
///
///  Entire dashed box is clickable
///  Accepts drag & drop inside the box
///  Hover transitions to darker background
///  Filename displayed inside the box
///  Hidden native file input
///  Optional "Open in New Tab" link
///
#[component]
pub fn FileUpload(
    #[prop()] file_types: Option<Vec<String>>, // Allowed extensions (.pdf, .docx)
    #[prop(into)] on_change: Callback<FileInfo>, // Callback for parent
    #[prop(optional, into)] label: String,
) -> impl IntoView {
    // File contents stored for preview
    let file_data = RwSignal::new(None::<Vec<u8>>);

    // Currently selected filename
    let file_name = RwSignal::new(None::<String>);

    // Convert Vec<String> → ".pdf, .docx" for <input accept="">
    let accept_value = file_types.clone().map(|v| v.join(",")).unwrap_or_default();

    // Prevent browser from opening the file when dragging over the box
    let drag_over = move |ev: DragEvent| ev.prevent_default();

    // Label text inside the dashed box
    let file_label = move || {
        file_name
            .get()
            .unwrap_or_else(|| "Click or drag a file to upload".to_string())
    };

    // Handle drag & drop uploads
    let drop = {
        let file_types = file_types.clone();
        let on_change = on_change.clone();
        let file_data = file_data;
        let file_name = file_name;

        move |ev: DragEvent| {
            ev.prevent_default();

            if let Some(dt) = ev.data_transfer() {
                if let Some(list) = dt.files() {
                    // Support dropping multiple files
                    for i in 0..list.length() {
                        if let Some(file) = list.get(i) {
                            process_file(file, &file_types, &on_change, file_data, file_name);
                        }
                    }
                }
            }
        }
    };

    // Handle file selection via clicking the box
    let on_change_input = {
        let file_types = file_types.clone();
        let on_change = on_change.clone();
        let file_data = file_data;
        let file_name = file_name;

        move |ev: Event| {
            let input: HtmlInputElement = event_target(&ev);

            // Only process the first selected file
            if let Some(file) = input.files().and_then(|fs| fs.get(0)) {
                process_file(file, &file_types, &on_change, file_data, file_name);
            }
        }
    };

    // Component View
    view! {
        <div class="file-upload flex flex-col flex-1">
        <span class="block ml-1.5 mb-0 font-bold">{label}</span>
            // Entire dashed box acts as upload area AND button
            <label
                class="file-upload-area block m-1.5 p-1.5 mt-0
                       border-2 border-dashed border-red-700 rounded-md
                       py-2 px-4 text-center cursor-pointer
                       transition-colors duration-150
                       bg-white hover:bg-gray-100"
                on:dragover=drag_over
                on:drop=drop
            >
                // Hidden file input triggered by clicking the label
                <input
                    type="file"
                    on:change=on_change_input
                    prop:accept=accept_value
                    style="display: none;"
                />

                // Centered text inside the upload box
                <div class="flex flex-col items-center justify-center gap-1">
                    <span class="text-sm font-medium">
                        {file_label}
                    </span>

                    <span class="text-xs text-gray-500">
                        "Supported types: "
                        {move || file_types
                            .clone()
                            .map(|v| v.join(", "))
                            .unwrap_or_else(|| "any".to_string())
                        }
                    </span>
                </div>
            </label>

            // Show preview button only when file bytes exist
            <Show when=move || file_data.get().is_some()>
                <div class="mt-2">
                    <button
                        class="text-sm underline text-blue-700 hover:text-blue-900"
                        on:click=move |_| {
                            if let Some(bytes) = file_data.get() {
                                open_in_new_tab(bytes, "application/octet-stream");
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

/// Open the uploaded file in a new browser tab.
/// Uses Blob + object URL.
pub fn open_in_new_tab(bytes: Vec<u8>, mime: &str) {
    let uint8 = js_sys::Uint8Array::from(bytes.as_slice());

    // Build Blob
    let mut bag = BlobPropertyBag::new();
    bag.set_type(mime);

    let blob =
        Blob::new_with_u8_array_sequence_and_options(&js_sys::Array::of1(&uint8), &bag).unwrap();

    // Convert Blob → temporary object URL
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Open in new tab
    window()
        .unwrap()
        .open_with_url_and_target(&url, "_blank")
        .unwrap();
}
