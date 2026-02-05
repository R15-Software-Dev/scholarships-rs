use crate::common::ValueType;
use leptos::ev::{Event, Targeted};
use leptos::prelude::*;
use leptos::web_sys::HtmlSelectElement;
use std::collections::HashMap;
use crate::components::{use_validation_context, InputState, ValidationState};

fn validate(required: bool, value: String) -> ValidationState {
    if required && (value.is_empty()) {
        ValidationState::Invalid("This input is required.".to_string())
    } else {
        ValidationState::Valid
    }
}

/// A custom-styled select input.
///
/// Takes a `Vec<String>` as the set of inputs. Only one value may be selected, which is indicated
/// by the `value` prop of the component.
#[component]
pub fn Select(
    #[prop(into)] value_list: Signal<Vec<String>>,
    #[prop(into)] data_member: Signal<String>,
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>
) -> impl IntoView {
    //#region Value Logic
    
    let value = Memo::new(move |_| {
        data_map.with(|map| {
            map.get(&data_member.get_untracked())
                .and_then(|v| v.as_string().ok().flatten())
                .unwrap_or_default()
        })
    });

    //#endregion
    //#region Validation Logic

    let error = Memo::new(move |_| validate(required.get(), value.get()));
    let dirty = RwSignal::new(false);
    let show_errors = Signal::derive(move || dirty.get() && matches!(error.get(), ValidationState::Invalid(_)));

    let input_state = RwSignal::new(InputState::new(data_member.get(), error.clone(), dirty.clone()));

    let validation_context = use_validation_context()
        .expect("Could not find FormValidationRegistry context");

    validation_context.validators.update(|list| list.push(input_state));

    on_cleanup(move || {
        validation_context.validators.update(|list| {
            list.retain(|v| *v.get_untracked().input_name != *input_state.get_untracked().input_name)
        });
    });

    //#endregion
    //#region Event Logic
    
    let on_change = move |e: Targeted<Event, HtmlSelectElement>| {
        let value = e.target().value();
        dirty.set(true);
        data_map.update(|map| {
            map.insert(data_member.get(), ValueType::String(Some(value)));
        });
    };

    //#endregion

    view! {
        <div class="flex flex-col flex-1">
            <div class="flex flex-1">
                <label class="flex flex-col flex-1">
                    <span class="ml-1.5 mb-0 font-bold">{label}</span>
                    <select
                        class="relative flex-1 border-2 m-1.5 mt-0 p-1.5 rounded-md
                        transition-border duration-150
                        border-red-700 bg-transparent
                        disabled:border-gray-600 disabled:pointer-events-none disabled:bg-gray-600/33"
                        prop:value=move || value.get()
                        on:change:target=on_change
                        disabled=disabled
                    >
                        // This closure handles the display of the options.
                        <option value="" disabled hidden selected=move || value.get().is_empty()>
                            "Select one..."
                        </option>

                        // We can reasonably assume that the options in the list will be distinct.
                        <For
                            each=move || value_list.get()
                            key=|value| value.clone()
                            children=move |val| view! { <option value=val>{val.clone()}</option> }
                        />
                    </select>
                </label>
            </div>
            <Show when=move || show_errors.get()>
                <div class="text-red-600 text-sm mr-1.5 ml-1.5">
                    {move || {
                        match error.get() {
                            ValidationState::Invalid(msg) => msg,
                            _ => "There is no error - should not see this message.".to_string(),
                        }
                    }}
                </div>
            </Show>
        </div>
    }
}
