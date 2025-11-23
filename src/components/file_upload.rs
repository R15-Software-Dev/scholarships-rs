use base64::Engine;
use leptos::ev::{DragEvent, Event};
use leptos::logging;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use web_sys::{File, HtmlAnchorElement, HtmlInputElement};

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
/// Shared handler for both:
///  - drag & drop files
///  - file input selection
///
fn process_file(
    file: File,
    allowed: &Option<Vec<String>>,
    on_change: &Callback<FileInfo>,
    file_data: RwSignal<Option<Vec<u8>>>,
    file_name: RwSignal<Option<String>>,
    error_message: RwSignal<Option<String>>,
) {
    let name = file.name();
    let mime = file.type_();
    let size = file.size() as u64;

    // Enforce extension whitelist (if provided)
    if let Some(allowed_types) = allowed {
        let ok = allowed_types.iter().any(|ext| name.ends_with(ext));
        if !ok {
            let allowed_str = allowed_types.join(", ");
            logging::log!("Rejected file due to extension mismatch: {}", name);

            // Clear any previous good file
            file_data.set(None);
            file_name.set(None);

            // Set visible error message
            error_message.set(Some(format!(
                "File \"{}\" was rejected. This field only accepts: {}",
                name, allowed_str
            )));
            return;
        }
    }

    // clear any previous error if this file is valid
    error_message.set(None);

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

    // Error message to show under the box
    let error_message = RwSignal::new(None::<String>);

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
        let error_message = error_message;

        move |ev: DragEvent| {
            ev.prevent_default();

            if let Some(dt) = ev.data_transfer() {
                if let Some(list) = dt.files() {
                    // Support dropping multiple files
                    for i in 0..list.length() {
                        if let Some(file) = list.get(i) {
                            process_file(
                                file,
                                &file_types,
                                &on_change,
                                file_data,
                                file_name,
                                error_message, // NEW
                            );
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
        let error_message = error_message;

        move |ev: Event| {
            let input: HtmlInputElement = event_target(&ev);

            if let Some(file) = input.files().and_then(|fs| fs.get(0)) {
                process_file(
                    file,
                    &file_types,
                    &on_change,
                    file_data,
                    file_name,
                    error_message,
                );
            }
        }
    };

    // Component View
    view! {
        <div class="file-upload flex flex-col flex-1">
            <span class="block ml-1.5 mb-0 font-bold">{label}</span>

            // Entire dashed box acts as upload area
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

            // Error message shown directly below the upload box
            <Show when=move || error_message.get().is_some()>
                <p class="mt-1 ml-1.5 text-sm text-red-600">
                    {move || error_message.get().unwrap_or_default()}
                </p>
            </Show>

            // Show preview button only when file bytes exist
            <Show when=move || file_data.get().is_some()>
                <div class="mt-2">
                    <button
                        class="text-sm underline text-blue-700 hover:text-blue-900"
                        on:click=move |_| {
                            if let Some(bytes) = file_data.get() {
                                open_in_new_tab(bytes, "application/pdf");
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
pub fn open_in_new_tab(bytes: Vec<u8>, mime: &str) {
    let base64 = base64::engine::general_purpose::STANDARD.encode(bytes);
    let data_url = format!("data:{};base64,{}", mime, base64);

    let link = document()
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    link.set_href(&*data_url);
    link.set_target("_blank");
    link.click();
}
