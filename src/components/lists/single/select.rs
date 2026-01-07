use crate::common::ValueType;
use leptos::ev::{Event, Targeted};
use leptos::prelude::*;
use leptos::web_sys::HtmlSelectElement;
use std::collections::HashMap;
use leptos::logging::{log, warn};
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
    #[prop()] value_list: Vec<String>,
    #[prop(into)] data_member: Signal<String>,
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] required: Signal<bool>
) -> impl IntoView {
    //#region Value Logic

    let default_raw_value = data_map.get_untracked()
        .get(&data_member.get_untracked())
        .cloned()
        .unwrap_or_default()
        .as_string().unwrap_or_default().unwrap_or_default();

    let raw_value = RwSignal::new(default_raw_value);

    Effect::new(move || {
        data_map.update(|map| {
            map.insert(data_member.get(), ValueType::String(Some(raw_value.get())));
        });
    });

    //#endregion
    //#region Validation Logic

    let error = RwSignal::new(
        validate(required.get_untracked(), raw_value.get_untracked())
    );
    warn!("Current error state: {:?}", error.get_untracked());
    let dirty = RwSignal::new(false);
    let show_errors = Signal::derive(move || dirty.get() && matches!(error.get(), ValidationState::Invalid(_)));

    let input_state = InputState::new(data_member.get(), error.clone(), dirty.clone());

    let validation_context = use_validation_context()
        .expect("Could not find FormValidationRegistry context");

    validation_context.validators.update(|list| list.push(RwSignal::new(input_state)));

    //#endregion
    //#region Event Logic

    let on_change = move |e: Targeted<Event, HtmlSelectElement>| {
        let value = e.target().value();
        log!("Found value {}", value);
        raw_value.set(value.clone());
        error.set(validate(required.get(), raw_value.get()));
        dirty.set(true);
        data_map.update(|map| {
            if let Some(val) = map.get_mut(&data_member.get()) {
                *val = ValueType::String(Some(value.clone()));
            } else {
                map.insert(data_member.get(), ValueType::String(Some(value)));
            }
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
                        on:change:target=on_change
                        disabled=disabled
                    >
                        // This closure handles the display of the options.
                        <option
                            value=""
                            disabled
                            hidden
                            selected={move || raw_value.get().is_empty()}
                        >"Select one..."</option>
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
            <Show when=move || show_errors.get()>
                <div class="text-red-600 text-sm mr-1.5 ml-1.5">
                    {move || {
                        match error.get() {
                            ValidationState::Invalid(msg) => msg,
                            _ => "There is no error - should not see this message.".to_string()
                        }
                    }}
                </div>
            </Show>
        </div>
    }
}
