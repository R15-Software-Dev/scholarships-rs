use leptos::prelude::*;

/// # Header Component
/// 
/// A simple header that may also contain a description. Should be used to annotate sections and
/// forms, with extra information about how that section/form will work.
/// 
/// Example usage:
/// ```
/// view! {
///     <Header
///         title: "Example header"
///         description: "Example description"
///     />
/// }
/// ```
#[component]
pub fn Header(
    /// The title of the header. Must always be present.
    #[prop(into)] title: Signal<String>,
    /// The description of the header. Hidden when nothing i s
    #[prop(optional, into)] description: Signal<String>,
) -> impl IntoView {
    let show_description = Signal::derive(move || {
        !description.get().is_empty()
    });
    
    view! {
        <div class="flex flex-col gap-2 m-1.5 mt-3 mb-5">
            <div class="text-3xl font-bold">
                {title}
            </div>
            <Show when=move || show_description.get()>
                <div>
                    {description}
                </div>
            </Show>
        </div>
    }
}
