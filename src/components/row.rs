use leptos::prelude::*;

/// # Row Component
///
/// This component is designed for use within the `Panel` component, however it may be used standalone
/// when used within any `div` with the `flex` property set.
///
/// Example usage (with `Panel`):
/// ```
/// view! {
///     <Panel>
///         <Row>
///             // More content here...
///         </Row>
///     </Panel>
/// }
/// ```
#[component]
pub fn Row(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-row gap-2 flex-1">
            {children()}
        </div>
    }
}
