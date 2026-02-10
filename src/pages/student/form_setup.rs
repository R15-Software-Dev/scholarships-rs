use std::collections::HashMap;
use leptos::prelude::*;
use crate::common::ValueType;
use crate::pages::api::students::{get_student_data, PutStudentData};
use crate::utils::get_user_claims;

pub struct StudentFormInfo {
    pub data_map: RwSignal<HashMap<String, ValueType>>,
    pub user_id: Memo<Option<String>>,
    pub submit_action: Callback<()>,
    pub refresh_trigger: Trigger,
    pub data_resource: Resource<Result<HashMap<String, ValueType>, ServerFnError>>
}

pub fn use_student_form(
    form_type: impl Into<Signal<String>>
) -> StudentFormInfo {
    let form_type = form_type.into();
    
    let user_claims = get_user_claims();
    let user_id = Memo::new(move |_| {
        user_claims.get()
            .map(|info| info.claims.subject.clone())
    });

    let data_map = RwSignal::new(HashMap::new());

    let refresh_trigger = Trigger::new();
    let data_resource = Resource::new(
        move || (user_id.get().unwrap_or_default(), form_type.get(), refresh_trigger.track()),
        async move |(id, form_type, _)| get_student_data(
            id,
            form_type,
        ).await
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
        submit_action.dispatch(PutStudentData {
            subject: user_id.get().unwrap_or_default(),
            data_type: form_type.get(),
            data_map: data_map.get()
        });
    };
    
    StudentFormInfo {
        data_map,
        refresh_trigger,
        data_resource,
        user_id,
        submit_action: on_submit.into(),
    }
}