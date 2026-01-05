use leptos::prelude::*;
use std::collections::HashMap;
use crate::common::ValueType;
use crate::components::{use_validation_context, InputState, ValidationState};

pub struct ListController {
    pub error: Signal<ValidationState>,
    pub dirty: RwSignal<bool>,
    pub show_errors: Signal<bool>,
    pub validator: RwSignal<InputState>,
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
    data_member: String,
    data_map: RwSignal<HashMap<String, ValueType>>,
    required: Signal<bool>
) -> ListController {
    //#region Central List Logic

    // Get default values from the data map. This will only run once, when the input is created
    // and mounted to the DOM.
    let temp_map = data_map.get_untracked();
    let origin_value_type = temp_map.get(&data_member)
        .unwrap_or(&ValueType::List(None));
    let origin_list = value_type_to_vec(origin_value_type);

    // "Raw" selected values - just strings, no ValueTyping
    let selected_list = RwSignal::new(origin_list);

    Effect::new({
        let data_member = data_member.clone();
        move || {
            // Convert selected strings into ValueTypes
            let typed_list = selected_list.with(|list|
                list.iter().cloned().map(|val|
                    ValueType::String(Some(val))
                ).collect::<Vec<ValueType>>()
            );

            // Update the map
            data_map.update(|map| {
                map.insert(data_member.clone(), ValueType::List(Some(typed_list)));
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
    let dirty = RwSignal::new(false);
    let show_errors = Signal::derive(move || dirty.get() && matches!(error.get(), ValidationState::Invalid(_)));

    let validator = RwSignal::new(InputState::new(
        data_member.clone(),
        error.clone(),
        dirty.clone()
    ));

    validation_context.validators.update(|list| list.push(validator.clone()));

    //#endregion

    ListController {
        error,
        dirty,
        show_errors,
        validator,
        selected_list,
    }
}
