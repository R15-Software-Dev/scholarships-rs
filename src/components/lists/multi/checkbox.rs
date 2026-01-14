use crate::common::ValueType;
use crate::components::utils::create_unique_id;
use leptos::prelude::*;
use std::collections::HashMap;
use super::utils::use_selectable_list;
use crate::components::ValidationState;

/// # Checkbox Component
/// This component should only be used from within a [`CheckboxList`] component.
///
/// It creates a single checkbox with a specific value. Indicating if it is checked is done
/// _externally_ from the component, and requires some sort of signal. The component itself
/// does not track whether it is checked, it only runs the `on_change` callback.
///
/// Example usage (from within a [`CheckboxList`]:
/// ```
/// let disabled_signal = RwSignal::new(false);
/// let checked_signal = Signal::derive(move || {
///     // It's recommended to use a derived signal of some sort, but if you want to create
///     // a manually controlled RwSignal, that's fine too.
///     true
/// });
///
/// view! {
///     <Checkbox
///         checked=checked_signal
///         on_change=move || log!("A callback that's run on change.")
///         name="some_name"
///         disabled=disabled_signal
///         value="Some Value"
///     />
/// }
/// ```
#[component]
pub fn Checkbox(
    /// A [`Signal`] indicating whether the checkbox is checked.
    #[prop()] checked: Signal<bool>,
    /// A callback that is run when the checkbox is checked and unchecked.
    #[prop(into)] on_change: Callback<(), ()>,
    /// The value of the checkbox.
    #[prop(into)] value: Signal<String>,
    /// The name of the checkbox. This is used to group checkboxes together, so every checkbox
    /// in a group should have the same name.
    #[prop(into)] name: Signal<String>,
    /// A [`Signal`] indicating whether the checkbox is disabled.
    #[prop(into)] disabled: Signal<bool>,
) -> impl IntoView {
    let id = create_unique_id(&name.get(), &value.get_untracked());

    view! {
        <label for=id class="flex items-center">
            <input
                // Using the "peer" class links this input with the div "peer" at the same level.
                // By using the "peer-checked" selector this div can toggle states based on this
                // input's state (checked/unchecked)
                class="
                relative peer shrink-0
                hidden"
                type="checkbox"
                id=id.clone()
                name=name.clone()
                on:change=move |_| on_change.run(())
                prop:checked=checked
                disabled=disabled
            />
            // This div displays the box for the checkbox, and applies some styling to the child
            // span element.
            <div class="relative inline-block h-5 w-5 m-1 mr-2 rounded-sm border border-gray-300 bg-white
            transition-bg duration-150 peer-checked:bg-red-700 peer-checked:border-red-700
            [&>span]:opacity-0 peer-checked:[&>span]:opacity-100
            peer-disabled:bg-gray-600/33 peer-disabled:border-gray-600
            peer-disabled:peer-checked:bg-gray-600">
                // This span is the check icon. It is centered within the div and has a border on the bottom
                // and right sides, which is then rotated to appear like a checkmark.
                <span class="absolute left-1/2 top-1/2 block h-3 w-1.5 -translate-x-1/2 -translate-y-1/2
                rotate-45 transform border-b-3 border-r-3
                border-white transition-opacity duration-150"></span>
            </div>
            <span>{value}</span>
        </label>
    }
}

/// # Checkbox List Component
/// This component creates a list of checkboxes, each with a specific value. The values are passed
/// through a prop rather than manually creating children.
///
/// Example usage:
/// ```
/// let items = vec! [
///     "Item 1",
///     "Item 2",
///     "Item 3"
/// ];
///
/// view! {
///     <CheckboxList
///         data_member="data_map_member"
///         data_map=RwSignal::new(HashMap::new())
///         items=items
///     />
/// }
/// ```
#[component]
pub fn CheckboxList(
    /// The data member that this list should edit/display from the `data_map`.
    #[prop(into)] data_member: Signal<String>,
    /// A [`Signal`] allowing access to a data store.
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    /// The available choices for this list.
    #[prop()] items: Vec<String>,
    /// A [`Signal`] indicating whether the checkbox list is disabled.
    #[prop(optional, into)] disabled: Signal<bool>,
    /// A label for the list. Shown like a field name in a form.
    #[prop(optional, into)] label: String,
    /// A [`Signal`] indicating whether the list is required. Used for validation.
    #[prop(optional, into)] required: Signal<bool>
) -> impl IntoView {
    let controller = use_selectable_list(
        data_member.clone(),
        data_map.clone(),
        required.clone()
    );

    //#region Render Logic
    view! {
        <div class="flex flex-col">
            <div class="m-1.5 mt-0 mb-0">
                <span class="font-bold">{label}</span>
                {items
                    .into_iter()
                    .map(|item| {
                        let item = Signal::derive({
                            let cloned = item.clone();
                            move || cloned.clone()
                        });

                        let checked = Signal::derive(move || {
                            controller.selected_list.get().contains(&item.get())
                        });

                        let on_change = move || {
                            // Update the data list by checking if the value is present in it. If so,
                            // remove it. If not, add it.
                            controller.selected_list.update(|list| {
                                let item = item.get();
                                if list.contains(&item) {
                                    list.retain(|val| *val != item);
                                } else {
                                    list.push(item);
                                }
                            });
                            controller.dirty.set(true);
                        };

                        view! {
                            <Checkbox
                                checked=checked
                                on_change=on_change
                                // The actual selected values are tracked by this element, not by the checkboxes themselves.
                                value=item
                                name=data_member
                                disabled=disabled
                            />
                        }
                    })
                    .collect_view()}
            </div>
            <Show when=move || controller.show_errors.get()>
                <div class="text-red-600 text-sm mr-1.5 ml-1.5">
                    {move || {
                        match controller.error.get() {
                            ValidationState::Invalid(msg) => msg,
                            _ => "There is no error - should not see this message.".to_string()
                        }
                    }}
                </div>
            </Show>
        </div>
    }
    //#endregion
}
