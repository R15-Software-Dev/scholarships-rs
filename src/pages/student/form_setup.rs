use crate::common::ValueType;
use crate::components::{Toast, ToastContext};
use crate::pages::api::students::{PutStudentData, get_student_data};
use crate::utils::get_user_claims;
use leptos::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

pub struct StudentFormInfo {
    pub data_map: RwSignal<HashMap<String, ValueType>>,
    pub user_id: Memo<Option<String>>,
    pub submit_action: Callback<()>,
    pub refresh_trigger: Trigger,
    pub data_resource: Resource<Result<HashMap<String, ValueType>, ServerFnError>>,
    pub submit_pending: Memo<bool>,
}

pub fn use_student_form(
    form_type: impl Into<Signal<String>>,
    enable_toasts: bool,
) -> StudentFormInfo {
    let form_type = form_type.into();

    let user_claims = get_user_claims();
    let user_id = Memo::new(move |_| user_claims.get().map(|info| info.claims.subject.clone()));

    let data_map = RwSignal::new(HashMap::new());

    let refresh_trigger = Trigger::new();
    let data_resource = Resource::new(
        move || (user_id.get(), form_type.get(), refresh_trigger.track()),
        async move |(id, form_type, _)| {
            let Some(id) = id else {
                return Ok(HashMap::new());
            };
            get_student_data(id, form_type).await
        },
    );

    Effect::new(move || {
        data_resource.with(|map_opt| {
            let Some(Ok(map)) = map_opt else {
                return;
            };

            data_map.set(map.to_owned());
        });
    });

    let submit_action = ServerAction::<PutStudentData>::new();
    let on_submit = move || {
        if let Some(id) = user_id.get() {
            submit_action.dispatch(PutStudentData {
                subject: id,
                data_type: form_type.get(),
                data_map: data_map.get(),
            });
        }
    };

    if enable_toasts {
        let mut toasts = expect_context::<ToastContext>();
        Effect::watch(
            move || submit_action.value().get(),
            move |value, _, _| {
                let Some(result) = value else {
                    return;
                };

                let toast = match result {
                    Ok(_) => Toast::new()
                        .id(Uuid::new_v4())
                        .header("Submission Successful")
                        .msg("You may continue editing or fill out another form."),
                    Err(e) => Toast::new()
                        .id(Uuid::new_v4())
                        .header("Submission Failed")
                        .msg(e.to_string()),
                };

                toasts.toast(toast);
                untrack(move || submit_action.clear());
            },
            false,
        );
    }

    StudentFormInfo {
        data_map,
        refresh_trigger,
        data_resource,
        user_id,
        submit_action: on_submit.into(),
        submit_pending: submit_action.pending(),
    }
}
