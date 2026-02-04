use std::collections::HashMap;
use leptos::logging::log;
use leptos::prelude::*;
use crate::common::ValueType;
use crate::components::ActionButton;
use crate::pages::api::students::{GetStudentData, PutStudentData};
use crate::utils::get_user_claims;

#[component]
pub fn StudentHomePage() -> impl IntoView {
    let user_claims = get_user_claims();
    let user_id = Memo::new(move |_| {
        user_claims.get()
            .map(|info| info.claims.subject.clone())
    });
    
    let create_student_data = ServerAction::<PutStudentData>::new();
    let get_student_data = ServerAction::<GetStudentData>::new();
    
    let on_click_student_test = move |_| {
        create_student_data.dispatch(PutStudentData {
            subject: user_id.get().unwrap_or_default(),
            data_type: "TESTING".into(),
            data_map: HashMap::from([
                ("testing_info".into(), ValueType::String(Some("hello!".to_string())))
            ]),
        });
    };
    
    let on_click_get_student = move |_| {
        get_student_data.dispatch(GetStudentData {
            subject: user_id.get().unwrap_or_default(),
            data_type: "TESTING".into()
        });
    };
    
    Effect::new(move || {
        let Some(value) = get_student_data.value().get() else {
            return;
        };
        
        log!("{:?}", value);
    });
    
    view! {
        <div class="flex flex-1 flex-col gap-4 items-center m-4">
            <div class="font-bold text-3xl">"Student Home Page"</div>
            <div class="text-lg">
                "This is just a placeholder for now. We'll show some redirect buttons."
            </div>
            <ActionButton on:click=on_click_student_test>"Test Put Student Function"</ActionButton>
            <ActionButton on:click=on_click_get_student>"Test Get Student Function"</ActionButton>
        </div>
    }
}
