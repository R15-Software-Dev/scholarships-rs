use crate::common::ValueType;
use leptos::ev::{Event, Targeted};
use leptos::prelude::*;
use leptos::web_sys::HtmlSelectElement;
use std::collections::HashMap;

/// A custom-styled select input.
///
/// Takes a `Vec<String>` as the set of inputs. Only one value may be selected, which is indicated
/// by the `value` prop of the component.
#[component]
pub fn Select(
    #[prop()] value_list: Vec<String>,
    #[prop(optional)] value: RwSignal<String>,
    #[prop(into)] data_member: String,
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let on_change = {
        let data_member = data_member.clone();
        move |e: Targeted<Event, HtmlSelectElement>| {
            let target_value = e.target().value();
            data_map.update(|map| {
                if let Some(val) = map.get_mut(&data_member) {
                    *val = ValueType::String(Some(target_value));
                } else {
                    map.insert(data_member.clone(), ValueType::String(Some(target_value)));
                }
            });
        }
    };

    view! {
        <div class="flex flex-1">
            <label class="flex flex-col flex-1">
                <span class="ml-1.5 mb-0 font-bold">{label}</span>
                <select
                    class="relative flex-1 border-2 m-1.5 mt-0 p-1.5 rounded-md
                        transition-border duration-150
                        border-red-700 bg-transparent
                        disabled:border-gray-600 disabled:pointer-events-none disabled:bg-gray-600/33"
                    on:change:target=on_change
                    disabled = disabled >
                    // This closure handles the display of the options.
                    {move || {
                        value_list.iter().map(
                            move |value| {
                                let display = value.to_owned();
                                view! { <option value={display}>{display.clone()}</option> }
                            }
                        ).collect::<Vec<_>>()
                    }}
                </select>
            </label>
        </div>
    }
}
