use crate::common::ValueType;
use crate::components::utils::create_unique_id;
use leptos::prelude::*;
use std::collections::HashMap;
use leptos::html::Input;
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
                class="relative peer shrink-0 hidden"
                type="radio"
                id=id.clone()
                name=name
                on:change=move |_| on_change.run(())
                prop:checked=checked
                disabled=disabled
            />
            // This div displays the box for the checkbox, and applies some styling to the child
            // span element.
            <div class="relative inline-block h-5 w-5 m-1 mr-2 rounded-full border border-gray-400 bg-white
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
            <span class="px-0.5 py-0.5 flex-1">{value.clone()}</span>
        </label>
    }
}

/// # Ratio Other Component
/// This is a simple "other" field that contains a text input component. It can be selected using the
/// same logic as the [`Radio`] component, and also has a secondary `RwSignal` that contains the
/// current value of the embedded text input.
///
/// Example usage: <still working on this>
#[component]
fn RadioOther(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<()>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(optional, into)] value: RwSignal<String>,
    #[prop(into)] dirty: RwSignal<bool>,
    #[prop(into)] placeholder: Signal<String>,
) -> impl IntoView {
    let id = create_unique_id(&name.get_untracked(), &"other".to_string());
    let input_ref = NodeRef::<Input>::new();

    let input_disabled = Signal::derive(move || {
        !checked.get() || disabled.get()
    });

    Effect::new(move || {
        checked.get()
            .then(|| {
                if let Some(node) = input_ref.get() {
                    node.focus().expect("Couldn't focus input element");
                }
            });
    });

    let on_change_text = move |ev| {
        let val = event_target_value(&ev);

        value.set(val);

        on_change.run(());
    };

    let on_blur_text = move |_| {
        dirty.set(true);
    };

    view! {
        <label for=id class="flex items_center">
            <input
                // Using the "peer" class links this input with the div "peer" at the same level.
                // By using the "peer-checked" selector this div can toggle states based on this
                // input's state (checked/unchecked)
                class="relative peer shrink-0 hidden"
                type="radio"
                id=id.clone()
                name=name
                on:change=move |_| on_change.run(())
                prop:checked=checked
                disabled=disabled
            />
            // This div displays the box for the checkbox, and applies some styling to the child
            // span element.
            <div class="relative inline-block min-h-5 min-w-5 h-5 w-5 m-1 mr-2 rounded-full border border-gray-400 bg-white
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
            <input
                node_ref=input_ref
                class="border-b-2 mb-1.5 px-1.5 py-0.5 rounded-t-xs bg-transparent relative flex-1 transition-all duration-150
                border-red-700 disabled:border-gray-600 disabled:pointer-events-none disabled:bg-gray-600/33"
                r#type="text"
                disabled=input_disabled
                placeholder=placeholder
                prop:value=value
                on:input=on_change_text
                on:blur=on_blur_text
            />
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
    #[prop(optional, into)] other_prompt: Signal<String>,
    #[prop(optional, into)] name: Signal<String>
) -> impl IntoView {
    //#region Value Logic

    // Store a single signal that is the currently selected value. We can only have one selected
    // value, so there's no list needed. The default selected value is whatever the map has selected
    // already, or nothing.

    let hydrated = RwSignal::new(false);
    let selected_value = RwSignal::new(String::new());
    let other_checked = RwSignal::new(false);
    let other_value = RwSignal::new(String::new());

    // Hydration effect that resolves data. Sets up all data properly.
    Effect::new({
        let items = items.clone();
        move || {
            if hydrated.get() {
                return;
            }

            let default_value = match data_map.get_untracked().get(&data_member.get_untracked()) {
                Some(ValueType::String(Some(value))) => value.clone(),
                _ => String::new()
            };

            if !items.contains(&default_value) && !default_value.is_empty() {
                other_checked.set(true);
                other_value.set(default_value.clone());
            }

            selected_value.set(default_value);

            hydrated.set(true);
        }
    });

    // When the other option is selected and the other value changes, update the selected value.
    Effect::new(move || {
        other_checked.get()
            .then(|| selected_value.set(other_value.get()));
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

    let input_state = RwSignal::new(InputState::new(
        data_member.get_untracked(),
        error.clone(),
        dirty.clone()
    ));

    validation_context.validators.update(|list| list.push(input_state));

    on_cleanup(move || {
        validation_context.validators.update(|list| {
            list.retain(|v| *v.get_untracked().input_name != *input_state.get_untracked().input_name)
        });
    });

    // When the selected value changes, update the data map at the correct location and validate.
    Effect::new(move || {
        error.set(validate(selected_value.get(), required.get()));
        data_map.update(|map| {
            map.insert(data_member.get(), ValueType::String(Some(selected_value.get())));
        });
    });

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
                            selected_value.get() == item.get() && !other_checked.get()
                        });
                        let on_change = move || {
                            other_checked.set(false);
                            selected_value.set(item.get());
                            error
                                .set(
                                    validate(
                                        selected_value.get_untracked(),
                                        required.get_untracked(),
                                    ),
                                );
                            dirty.set(true);
                        };
                        // on_change only runs when the radio button is checked, not when unchecked.
                        // Change the selected value to the new one.

                        view! {
                            <Radio
                                checked=checked_signal
                                on_change=on_change
                                // The actual selected values are tracked by this element, not by the checkboxes themselves.
                                value=item
                                name=move || format!("{}_{}", name.get(), data_member.get())
                                disabled=disabled
                            />
                        }
                    })
                    .collect::<Vec<_>>()}

                <Show when=move || !other_prompt.get().is_empty()>
                    <RadioOther
                        checked=other_checked
                        name=data_member
                        disabled=disabled
                        value=other_value
                        on_change=move |_| other_checked.set(true)
                        placeholder=other_prompt
                        dirty=dirty
                    />
                </Show>
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

    //#endregion
}
