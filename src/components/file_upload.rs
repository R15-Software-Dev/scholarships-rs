use leptos::ev::{DragEvent, Event};
use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::{Blob, BlobPropertyBag, File, HtmlInputElement, Url, window};

/// Represents metadata about an uploaded file.
/// Sent upward to the parent via `on_change`.
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

/// Status of a file during the upload lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileStatus {
    #[default]
    Pending,
    Uploading,
    Completed,
    Error,
    Cancelled,
}

/// Represents upload progress (for future expansion).
#[derive(Debug, Clone, PartialEq)]
pub struct UploadProgress {
    pub file_id: String,
    pub progress: f64,
    pub bytes_uploaded: u64,
    pub total_bytes: u64,
}

///
/// Process a file selected either via:
///  - drag & drop, OR
///  - selecting with the "Browse" button
///
/// This centralizes:
///  - extension whitelisting
///  - updating the displayed file name
///  - reading bytes (for the "Open in New Tab" button)
///  - notifying the parent component
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

    // 🔒 Enforce extension restrictions (.pdf, .docx, etc.)
    if let Some(allowed_types) = allowed {
        let ok = allowed_types.iter().any(|ext| name.ends_with(ext));
        if !ok {
            logging::log!("Rejected file: {}", name);
            file_data.set(None);
            file_name.set(None);
            return;
        }
    }

    // Save the filename so UI can display it.
    file_name.set(Some(name.clone()));

    // Read file into bytes for the "Open in New Tab" functionality.
    // Must be async because FileReader/arrayBuffer() is async.
    let fut = async move {
        let array_buf = wasm_bindgen_futures::JsFuture::from(file.array_buffer())
            .await
            .unwrap();

        let bytes = js_sys::Uint8Array::new(&array_buf).to_vec();
        Some(bytes)
    };

    // Spawn Leptos async task to load the file contents.
    spawn_local(async move {
        let bytes = fut.await;
        file_data.set(bytes);
    });

    // Construct FileInfo and notify the parent
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

    // Trigger parent callback
    on_change.run(info);
}

///
/// Main FileUpload component.
///
/// Provides:
///  - drag & drop file selection
///  - hidden native file input
///  - custom styled "Browse" button
///  - filename display that updates for BOTH browse & drag/drop
///  - "Open in New Tab" button
///
#[component]
pub fn FileUpload(
    #[prop()] file_types: Option<Vec<String>>, // List of allowed extensions (e.g., .pdf, .docx)
    #[prop(into)] on_change: Callback<FileInfo>, // Callback to parent component
) -> impl IntoView {
    // Stores file contents (used for `Open in New Tab`)
    let file_data = RwSignal::new(None::<Vec<u8>>);

    // Stores the file's display name (replaces browser's "No file selected")
    let file_name = RwSignal::new(None::<String>);

    // Turn allowed extensions into a string for <input accept="">
    let accept_value = file_types.clone().map(|v| v.join(",")).unwrap_or_default();

    // Prevent browser from opening the file on dragover
    let drag_over = move |ev: DragEvent| ev.prevent_default();

    // Function that determines what filename text should display.
    // Replaces the native "No file selected" text entirely.
    let file_label = move || {
        file_name
            .get()
            .unwrap_or_else(|| "No file selected".to_string())
    };

    // Handle drag & drop files.
    // Calls process_file() for each dropped file.
    let drop = {
        let file_types = file_types.clone();
        let on_change = on_change.clone();
        let file_data = file_data;
        let file_name = file_name;

        move |ev: DragEvent| {
            ev.prevent_default();

            if let Some(dt) = ev.data_transfer() {
                if let Some(list) = dt.files() {
                    // Loop through each dragged file
                    for i in 0..list.length() {
                        if let Some(file) = list.get(i) {
                            process_file(file, &file_types, &on_change, file_data, file_name);
                        }
                    }
                }
            }
        }
    };

    // Handle file input selection ("Browse" button).
    // Same logic as drag & drop — both paths call process_file().
    let on_change_input = {
        let file_types = file_types.clone();
        let on_change = on_change.clone();
        let file_data = file_data;
        let file_name = file_name;

        move |ev: Event| {
            let input: HtmlInputElement = event_target(&ev);

            // Only one file allowed (input.files().get(0))
            if let Some(file) = input.files().and_then(|fs| fs.get(0)) {
                process_file(file, &file_types, &on_change, file_data, file_name);
            }
        }
    };

    // Component UI
    view! {
        <div
            class="file-upload"
            on:dragover=drag_over
            on:drop=drop
        >
            <label class="file-upload-label"
                style="display: inline-flex; align-items: center; gap: 0.5rem; cursor: pointer;"
            >
                // Custom "Browse" button (styled, not native)
                <span class="file-upload-browse"
                    style="padding: 0.25rem 0.75rem; border: 1px solid #ccc; border-radius: 4px;"
                >
                    "Browse"
                </span>

                // Display the current selected filename
                <span class="file-upload-filename"
                    style="font-size: 0.875rem; color: #555;"
                >
                    {file_label}
                </span>

                // Hidden actual file input (browser UI hidden)
                <input
                    type="file"
                    on:change=on_change_input
                    prop:accept=accept_value
                    style="display: none;"
                />
            </label>

            // Show "Open in New Tab" only if file_data is loaded
            <Show when=move || file_data.get().is_some()>
                <div class="mt-2">
                    <button
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

/// Open a file in a new browser tab using Blob + URL.createObjectURL.
pub fn open_in_new_tab(bytes: Vec<u8>, mime: &str) {
    // Convert Rust Vec<u8> → JS Uint8Array
    let uint8 = js_sys::Uint8Array::from(bytes.as_slice());

    // Create JS Blob with MIME type
    let mut bag = BlobPropertyBag::new();
    bag.set_type(mime);

    let blob =
        Blob::new_with_u8_array_sequence_and_options(&js_sys::Array::of1(&uint8), &bag).unwrap();

    // Create a temporary object URL for browser to open
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Open new tab
    window()
        .unwrap()
        .open_with_url_and_target(&url, "_blank")
        .unwrap();
}
