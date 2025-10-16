use crate::common::ValueType;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Checkbox(
    #[prop()] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<(), ()>,
    #[prop(into)] value: String,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <label for=value class="flex items-center">
            <input
                // Using the "peer" class links this input with the div "peer" at the same level.
                // By using the "peer-checked" selector this div can toggle states based on this
                // input's state (checked/unchecked)
                class="
                relative peer shrink-0
                hidden"
                type="checkbox"
                id=value.clone()
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
            <span>{value.clone()}</span>
        </label>
    }
}

#[component]
pub fn CheckboxList(
    #[prop(optional)] selected: RwSignal<Vec<String>>,
    #[prop(into)] data_member: String,
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop()] items: Vec<String>,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
    #[prop(optional, into)] label: String,
) -> impl IntoView {
    view! {
        <div class="m-1.5 mt-0 mb-0">
            <span class="font-bold">{label}</span>
            {items
                .into_iter()
                .map(|item| {
                    let checked_signal = Signal::derive({
                        let item_name = item.clone();
                        let data_member = data_member.clone();
                        move || {
                            data_map.get()  // HashMap
                                .get(&data_member)  // Option<ValueType>
                                .unwrap_or(&ValueType::List(None))  // ValueType
                                .as_list()  // Result<Option<Vec<ValueType>>, ValueType>
                                .unwrap_or(Some(vec!()))  // Option<Vec<ValueType>>
                                .unwrap_or(vec!())  // Vec<ValueType>
                                .contains(&ValueType::String(Some(item_name.clone())))  // bool
                        }
                    });

                    let on_change = {
                        let item_name = item.clone();
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
                        <Checkbox
                            checked=checked_signal
                            on_change=on_change
                            // The actual selected values are tracked by this element, not by the checkboxes themselves.
                            value=item.clone()
                            disabled=disabled
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
