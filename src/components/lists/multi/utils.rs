use leptos::prelude::*;
use std::collections::HashMap;
use crate::common::ValueType;
use crate::components::{use_validation_context, InputState, ValidationState};

pub struct ListController {
    pub error: Signal<ValidationState>,
    pub dirty: RwSignal<bool>,
    pub show_errors: Signal<bool>,
    pub selected_list: RwSignal<Vec<String>>,
}

fn value_type_to_vec(value_type: &ValueType) -> Vec<String> {
    let ValueType::List(Some(list)) = value_type else {
        return Vec::new();
    };

    list.iter().map(|val| {
        val.as_string()
            .unwrap_or_default()
            .unwrap_or_default()
    }).collect::<Vec<String>>()
}

pub fn use_selectable_list(
    data_member: Signal<String>,
    data_map: RwSignal<HashMap<String, ValueType>>,
    required: Signal<bool>
) -> ListController {
    //#region Central List Logic

    // "Raw" selected values - just strings, no ValueTyping
    let selected_list = RwSignal::new(Vec::new());
    let hydrated = RwSignal::new(false);
    let dirty = RwSignal::new(false);

    // Hydration effect - runs only once, to hydrate data.
    Effect::new(move || {
        if hydrated.get() {
            return;
        }

        // React to data_map changes
        let map = data_map.get();
        let key = data_member.get();

        let list = map
            .get(&key)
            .unwrap_or(&ValueType::List(None));

        selected_list.set(value_type_to_vec(list));

        hydrated.set(true);
    });

    Effect::new({
        move || {
            // Convert selected strings into ValueTypes
            let typed_list = selected_list.with(|list|
                list.iter().cloned().map(|val|
                    ValueType::String(Some(val))
                ).collect::<Vec<ValueType>>()
            );

            // Update the map
            data_map.update(|map| {
                map.insert(data_member.get(), ValueType::List(Some(typed_list)));
            });
        }
    });

    //#endregion
    //#region Form Validation

    let validation_context = use_validation_context()
        .expect("FormValidationRegistry was not found");

    let error = Signal::derive(move || {
        if required.get() && selected_list.get().len() <= 0 {
            ValidationState::Invalid("This field is required.".to_string())
        } else {
            ValidationState::Valid
        }
    });

    let show_errors = Signal::derive(move || dirty.get() && matches!(error.get(), ValidationState::Invalid(_)));

    let validator = RwSignal::new(InputState::new(
        data_member.get(),
        error.clone(),
        dirty.clone()
    ));

    validation_context.validators.update(|list| list.push(validator.clone()));

    //#endregion

    ListController {
        error,
        dirty,
        show_errors,
        selected_list,
    }
}
