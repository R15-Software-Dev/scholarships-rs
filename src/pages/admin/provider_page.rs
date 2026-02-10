use leptos::logging::log;
use leptos::prelude::*;
use crate::common::ValueType;
use crate::components::{NavListInfo, SelectableNavList};
use crate::pages::api::get_all_providers;

#[component]
pub fn AdminProviderPage() -> impl IntoView {
    let selected_items = RwSignal::new(Vec::new());
    let provider_list = RwSignal::new(Vec::new());

    let provider_trigger = Trigger::new();
    let provider_resource = Resource::new(
        move || provider_trigger.track(),
        async move |_| {
            get_all_providers().await
        }
    );

    Effect::new(move || {
        if let Some(Err(err)) = provider_resource.get() {
            log!("Found error: {}", err.to_string());
        }
        let Some(Ok(list)) = provider_resource.get() else {
            return;
        };

        let members_list = vec![
            "contact_email".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
        ];

        // Convert the list into a series of NavListInfo
        let converted = list
            .into_iter()
            .map(|info| {
                let mut text_list = vec![];
                members_list.iter().for_each(|member| {
                    let val = info.get(member)
                        .unwrap_or(&ValueType::String(None))
                        .as_string()
                        .ok().flatten()
                        .unwrap_or_default();
                    text_list.push(val);
                });

                let subject = info.get("subject")
                    .unwrap_or(&ValueType::String(None))
                    .to_string();

                NavListInfo {
                    text: text_list,
                    id: subject.clone(),
                    slug: subject,
                }
            })
            .collect();

        provider_list.set(converted);
    });

    let headers = vec![
        "Email",
        "First Name",
        "Last Name"
    ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    
    view! {
        <div class="flex flex-1 flex-col gap-4 items-stretch mt-4">
            <div class="font-bold text-3xl self-center">"Provider View"</div>
            <SelectableNavList
                selected=selected_items
                name="providers-nav-list"
                items=provider_list
                headers=headers
            />
        </div>
    }
}
