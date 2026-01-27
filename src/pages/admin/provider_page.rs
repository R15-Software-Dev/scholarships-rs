use leptos::prelude::*;
use crate::components::{NavListInfo, SelectableNavList};

#[component]
pub fn AdminProviderPage() -> impl IntoView {
    let selected_items = RwSignal::new(Vec::new());
    let provider_list = RwSignal::new(vec![
        NavListInfo {
            text: vec!["Testing 1".to_string(), 3.to_string(), 500.to_string()],
            id: "1".to_string(),
            slug: "1".to_string(),
        },
        NavListInfo {
            text: vec!["Testing 2".to_string(), 2.to_string(), 1000.to_string()],
            id: "2".to_string(),
            slug: "2".to_string(),
        }
    ]);

    let headers = vec![
        "Scholarship Name",
        "Number of Awards",
        "Amount of Awards"
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
