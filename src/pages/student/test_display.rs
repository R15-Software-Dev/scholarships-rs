use crate::components::{Loading, ValueDisplay};
use crate::pages::api::students::get_all_student_data;
use crate::utils::get_user_claims;
use leptos::prelude::*;

#[component]
pub fn StudentTestDisplay() -> impl IntoView {
    let user_claims = get_user_claims();
    let user_id = Memo::new(move |_| user_claims.get().map(|info| info.claims.subject.clone()));

    let resource = Resource::new(
        move || user_id.get().unwrap_or_default(),
        async move |id| get_all_student_data(id).await,
    );

    view! {
        <Suspense fallback=Loading>
            {move || {
                resource
                    .get()
                    .map(|data_map| {
                        let Ok(map) = data_map else {
                            return view! {}.into_any();
                        };
                        let data_map = StoredValue::new(map);

                        view! {
                            <div>
                                <For
                                    each=move || data_map.get_value()
                                    key=|(k, v)| k.clone()
                                    children=move |(k, v)| {
                                        let member = StoredValue::new(k.clone());

                                        view! {
                                            <ValueDisplay
                                                data_map=data_map.get_value()
                                                data_member=member.get_value()
                                                header=member.get_value()
                                            />
                                        }
                                    }
                                />
                            </div>
                        }
                            .into_any()
                    })
            }}
        </Suspense>
    }
}
