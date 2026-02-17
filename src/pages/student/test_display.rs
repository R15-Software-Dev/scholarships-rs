use crate::components::{Loading, ValueDisplay};
use crate::pages::api::students::get_all_student_data;
use crate::utils::get_user_claims;
use leptos::prelude::*;
use crate::common::{SchemaNode, SchemaType};

#[component]
pub fn StudentTestDisplay() -> impl IntoView {
    let create_student_display_schema = move || {
        SchemaNode::new(SchemaType::Map)
            .header("Student Information:")
            .child("first_name", SchemaNode::new(SchemaType::String).header("First Name:"))
            .child("last_name", SchemaNode::new(SchemaType::String).header("Last Name:"))
            .child("sports_participation", SchemaNode::new(SchemaType::List)
                .header("List of Sports Activities")
                .item_template(SchemaNode::new(SchemaType::Map)
                    .child("sport_name", SchemaNode::new(SchemaType::String).header("Sport Name:"))
                    .child("achievements", SchemaNode::new(SchemaType::String).header("Special Achievements"))
                )
            )
    };

    let user_claims = get_user_claims();
    let user_id = Memo::new(move |_| user_claims.get().map(|info| info.claims.subject.clone()));

    let resource = Resource::new(
        move || user_id.get().unwrap_or_default(),
        async move |id| get_all_student_data(id).await,
    );

    let schema = StoredValue::new(create_student_display_schema());

    view! {
        <Suspense fallback=Loading>
            <div class="flex flex-1 flex-col">
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
                                    <ValueDisplay
                                        schema=schema.get_value()
                                        data_map=data_map.get_value()
                                    />
                                </div>
                            }
                                .into_any()
                        })
                }}
            </div>
        </Suspense>
    }
}
