use leptos::prelude::*;

/// A panel for use in visual separation of a group of components. A `Panel` arranges
/// a series of elements within a shaded rectangle using the flexbox column layout.
///
/// It takes a series of children. Usually these would be in the form of a `Row` element,
/// which will arrange its own children in a row within the panel, however it can be any
/// HTML element.
///
/// Example:
/// ```
/// view! {
///     <Panel>
///         <Row>
///             <OutlinedTextField
///                 // ...
///             />
///             <CheckboxList
///                 // ...
///             />
///         </Row>
///     </Panel>
/// }
/// ```
#[component]
pub fn Panel(
    #[prop(optional, default = false)] hidden: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            class="flex flex-col gap-2 p-2 m-4 flex-2
                border-1 border-gray-600/10
                rounded-md shadow-[0_0_6px_rgba(0,0,0,0.50)]"
            class=(["hidden"], hidden) >
            {children.and_then(move |t| {Some(t())})}
        </div>
    }
}
