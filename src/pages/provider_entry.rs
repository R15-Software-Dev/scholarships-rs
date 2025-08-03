use crate::components::{MultiEntry, MultiEntryData, MultiEntryMember, OutlinedTextField};
use leptos::prelude::*;

#[component]
pub fn ProviderEntry() -> impl IntoView {
    let multi_entries: RwSignal<Vec<MultiEntryData>> = RwSignal::new(vec![]);
    
    view! (
        <h1>This is the provider entry page</h1>
        <h2>It would usually have a series of questions that we ask the providers</h2>
        <div>
            <OutlinedTextField
                placeholder = "Testing information...".into()
                name = "testing_input".into()
                label = "This is a testing question.".into()
            />
        </div>
        <div>
            <p>This is an example of the multientry element.</p>
            <MultiEntry
                entries=multi_entries
                name_member=Signal::from(MultiEntryMember::from_str("Entry Name", "name"))
                info_member=Signal::from(MultiEntryMember::from_str("Entry Info", "info"))
            />
        </div>
    )
}
