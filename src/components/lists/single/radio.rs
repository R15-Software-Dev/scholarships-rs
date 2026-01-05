use crate::common::ValueType;
use crate::components::utils::create_unique_id;
use leptos::prelude::*;
use std::collections::HashMap;
use crate::components::{use_validation_context, InputState, ValidationState};

#[component]
pub fn Radio(
    #[prop()] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<(), ()>,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] name: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let id = create_unique_id(&name.get_untracked(), &value.get_untracked());

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
                name=name
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

fn validate(value: String, required: bool) -> ValidationState {
    if required && value.is_empty() {
        ValidationState::Invalid("This field is required".into())
    } else {
        ValidationState::Valid
    }
}

#[component]
pub fn RadioList(
    #[prop(into)] data_member: Signal<String>,
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    #[prop()] items: Vec<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] required: Signal<bool>,
) -> impl IntoView {
    //#region Value Logic

    // Store a single signal that is the currently selected value. We can only have one selected
    // value, so there's no list needed. The default selected value is whatever the map has selected
    // already, or nothing.
    let default_value = match data_map.get_untracked().get(&data_member.get_untracked()) {
        Some(ValueType::String(Some(value))) => value.clone(),
        _ => String::new()
    };

    let selected_value = RwSignal::new(default_value);

    // When the selected value changes, update the data map at the correct location
    Effect::new(move || {
        data_map.update(|map| {
            map.insert(data_member.get(), ValueType::String(Some(selected_value.get())));
        });
    });

    //#endregion
    //#region Validation Logic

    let validation_context = use_validation_context()
        .expect("Could not find FormValidationRegistry context");

    let error = RwSignal::new(validate(selected_value.get_untracked(), required.get_untracked()));
    let dirty = RwSignal::new(false);
    let show_errors = Signal::derive(move || {
        dirty.get() && matches!(error.get(), ValidationState::Invalid(_))
    });

    let input_state = InputState::new(
        data_member.get_untracked(),
        error.clone(),
        dirty.clone()
    );

    validation_context.validators.update(|list| list.push(RwSignal::new(input_state)));

    //#endregion
    //#region Render Logic

    view! {
        <div class="flex flex-col flex-1">
            <div class="m-1.5 mt-0 mb-0">
                <span class="font-bold">{label}</span>
                {items
                    .into_iter()
                    .map(|item| {
                        let item = RwSignal::new(item);

                        let checked_signal = Signal::derive(move || {
                            selected_value.get() == item.get()
                        });

                        let on_change = move || {
                            // on_change only runs when the radio button is checked, not when unchecked.
                            // Change the selected value to the new one.
                            selected_value.set(item.get());
                            error.set(validate(selected_value.get_untracked(), required.get_untracked()));
                            dirty.set(true);
                        };

                        view! {
                            <Radio
                                checked=checked_signal
                                on_change=on_change
                                // The actual selected values are tracked by this element, not by the checkboxes themselves.
                                value=item
                                name=data_member
                                disabled=disabled
                            />
                        }
                    })
                    .collect::<Vec<_>>()}
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

    //#endregion
}
