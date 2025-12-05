use crate::common::ValueType;
use crate::components::utils::create_unique_id;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Radio(
    #[prop()] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<(), ()>,
    #[prop(into)] value: String,
    #[prop(into)] name: String,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
) -> impl IntoView {
    let id = create_unique_id(&name, &value);

    view! {
        <label for=id class="flex items-center">
            <input
                // Using the "peer" class links this input with the div "peer" at the same level.
                // By using the "peer-checked" selector this div can toggle states based on this
                // input's state (checked/unchecked)
                class="
                relative peer shrink-0
                hidden"
                type="radio"
                id=id.clone()
                name=name.clone()
                on:change=move |_| on_change.run(())
                prop:checked=checked
                disabled=disabled
            />
            // This div displays the box for the checkbox, and applies some styling to the child
            // span element.
            <div class="relative inline-block h-5 w-5 m-1 mr-2 rounded-full border border-gray-300 bg-white
            transition-bg duration-200 peer-checked:bg-red-700 peer-checked:border-red-700
            [&>span]:opacity-0 peer-checked:[&>span]:opacity-100
            peer-disabled:bg-gray-600/33 peer-disabled:border-gray-600
            peer-disabled:peer-checked:bg-gray-600">
                // This span is the check icon. It is centered within the div and has a border on the bottom
                // and right sides, which is then rotated to appear like a checkmark.
                <span class="absolute left-1/2 top-1/2 block h-2.5 w-2.5 -translate-x-1/2 -translate-y-1/2
                rotate-45 transform border-3 border-3 rounded-full bg-white
                border-white transition-opacity duration-150"></span>
            </div>
            <span>{value.clone()}</span>
        </label>
    }
}

#[component]
pub fn RadioList(
    // #[prop(default = RwSignal::new(String::new()))] selected: RwSignal<ValueType>,
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
                            // Check if the value is contained in the data map's fields
                            let map = data_map.get();
                            let value = map.get(&data_member);
                            let mut result = false;
                            if let Some(val) = value {
                                if val.is_string() {
                                    result = val.as_string().unwrap().unwrap() == item_name
                                }
                            }

                            result
                        }
                    });

                    let on_change = {
                        let item_name = item.clone();
                        let data_member = data_member.clone();
                        move || {
                            // Update the hash map
                            data_map.update(|map| {
                                if let Some(val) = map.get_mut(&data_member) {
                                    *val = ValueType::String(Some(item_name.clone()));
                                } else {
                                    map.insert(data_member.clone(), ValueType::String(Some(item_name.clone())));
                                }
                            });
                        }
                    };

                    view! {
                        <Radio
                            checked=checked_signal
                            on_change=on_change
                            // The actual selected values are tracked by this element, not by the checkboxes themselves.
                            value=item.clone()
                            name=data_member.clone()
                            disabled=disabled
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
