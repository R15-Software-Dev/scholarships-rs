use crate::common::ValueType;
use crate::components::utils::create_unique_id;
use leptos::prelude::*;
use std::collections::HashMap;
use crate::components::ValidationState;
use super::utils::use_selectable_list;

/// # Chip Component
///
/// This component defines a single chip for use in the `ChipsList` component. It functions almost
/// identically to the `Checkbox` component. It should only be created from within its parent
/// `CheckboxList` component.
///
/// Example usage:
/// ```
/// view! {
///     <Chip
///         value="Some Value"
///         name="list_name"
///         on_change=move || { println!("Some work here.") }
///         checked=RwSignal::new(false)
///         disabled=RwSignal::new(false)
///     />
/// }
/// ```
#[component]
pub fn Chip(
    #[prop(into)] display_text: Signal<String>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] name: String,
    #[prop(into)] on_change: Callback<(), ()>,
    #[prop()] checked: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    // Generate a unique id - ensure that the value doesn't contain spaces.
    // Without this id, checkbox/radio inputs can interfere with each other.
    let id = create_unique_id(&name, &value.get_untracked());

    view! {
        <label for=id>
            <input
                r#type="checkbox"
                class="relative peer shrink-0 hidden"
                name=name.clone()
                id=id.clone()
                prop:checked=checked
                on:change=move |_| on_change.run(())
                disabled=disabled />
            <div class="rounded-full p-2.5
                cursor-pointer text-base
                bg-transparent border-1 border-gray-300 
                shadow-md/50 hover:shadow-md/70
                peer-checked:bg-red-700 peer-checked:border-red-700 peer-checked:text-white
                peer-disabled:peer-checked:bg-gray-700
                peer-disabled:peer-checked:border-gray-700
                peer-disabled:cursor-default
                transition-all duration-150">
                <span>{display_text.clone()}</span>
            </div>
        </label>
    }
}

/// # Chips List Component
///
/// This defines a list of `Chip` components. The list of values is created based on the data from the
/// `data_map` prop, which is found using the given `data_member`.
///
/// Any values selected from the list are stored within the `data_map` immediately as part of a callback
/// passed to the `on_change` prop for each individual `Chip` component.
#[component]
pub fn ChipsList(
    #[prop(into)] data_member: String,
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop(into)] displayed_text: Signal<Vec<String>>,
    #[prop(into)] values: Signal<Vec<String>>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] required: Signal<bool>
) -> impl IntoView {
    let controller = use_selectable_list(
        data_member.clone(),
        data_map.clone(),
        required.clone()
    );

    view! {
        <div class="flex flex-col">
            <div class="m-1.5 mt-0 mb-0">
                <span class="font-bold">{label}</span>
                {displayed_text.get().into_iter().zip(values.get())
                    .into_iter()
                    .map(|(text, value)| {
                        let value = RwSignal::new(value);

                        let checked = Signal::derive(move || {
                            controller.selected_list.get().contains(&value.get())
                        });

                        let on_change = move || {
                            // Update the data list by checking if the value is present in it. If so,
                            // remove it. If not, add it.
                            controller.selected_list.update(|list| {
                                let value = value.get();
                                if list.contains(&value) {
                                    list.retain(|val| *val != value);
                                } else {
                                    list.push(value.clone());
                                }
                            });
                            controller.dirty.set(true);
                        };

                        view! {
                            <Chip
                                checked=checked
                                on_change=on_change
                                value=value
                                display_text=text
                                name=data_member.clone()
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
}
