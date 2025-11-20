use std::collections::HashMap;
use leptos::prelude::*;
use crate::common::ValueType;

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
    #[prop(into)] display_text: String,
    #[prop(into)] value: String,
    #[prop(into)] name: String,
    #[prop(into)] on_change: Callback<(), ()>,
    #[prop()] checked: Signal<bool>,
    #[prop(optional)] disabled: RwSignal<bool>,
) -> impl IntoView {
    // Generate a unique id - ensure that the value doesn't contain spaces.
    // Without this id, checkbox/radio inputs can interfere with each other.
    let value_no_spaces = value.clone().chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    let id = format!("{}-{}", name.clone(), value_no_spaces);
    
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
    #[prop()] displayed_text: Vec<String>,
    #[prop()] values: Vec<String>,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
    #[prop(optional, into)] label: String,
) -> impl IntoView {
    view! {
        <div class="m-1.5 mt-0 mb-0">
            <span class="font-bold">{label}</span>
            <div class="flex flex-row flex-wrap gap-2 mt-1">
                {displayed_text.into_iter().zip(values)
                    .into_iter()
                    .map(|(text, value)| {
                        let checked_signal = Signal::derive({
                            let item_name = value.clone();
                            let data_member = data_member.clone();
                            move || {
                                data_map.get()
                                    .get(&data_member)
                                    .unwrap_or(&ValueType::List(None))
                                    .as_list()
                                    .unwrap_or(Some(vec!()))
                                    .unwrap_or(vec!())
                                    .contains(&ValueType::String(Some(item_name.clone())))
                            }
                        });
                    
                        let on_change = {
                            let item_name = value.clone();
                            let data_member = data_member.clone();
                            move || {
                                let result = ValueType::String(Some(item_name.clone()));
                                data_map.update(|map| {
                                    // Attempt to get the value type from the hash map.
                                    match map.get_mut(&data_member) {
                                        Some(value_type) => {
                                            // Attempt to get the value as a list.
                                            match value_type.as_list().unwrap_or(None) {
                                                Some(mut list) => {
                                                    // Attempt to find the selected value in the selected list.
                                                    // If it's present, remove it, or vice versa.
                                                    match list.iter().position(|val| *val == result) {
                                                        Some(index) => { list.remove(index); },
                                                        None => { list.push(result); }
                                                    };
                                                    // Update the existing value_type entry in the hash map.
                                                    *value_type = ValueType::List(Some(list));
                                                },
                                                // Insert a new value_type entry.
                                                None => { *value_type = ValueType::List(Some(vec![result])); }
                                            };
                                        },
                                        // Insert a new value_type entry.
                                        None => { map.insert(data_member.clone(), ValueType::List(Some(vec![result]))); }
                                    };
                                });
                            }
                        };
                    
                        view! {
                            <Chip
                                checked=checked_signal
                                name=data_member.clone()
                                on_change=on_change
                                value=value.clone()
                                display_text=text.clone()
                                disabled=disabled
                            />
                        }
                    })
                    .collect_view()
                }
            </div>
        </div>
    }
}
