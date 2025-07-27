use crate::components::{MultiEntry, OutlinedTextField};
use leptos::prelude::*;

#[component]
pub fn ProviderEntry() -> impl IntoView {
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
                nameMember="name".into()
                infoMember="info".into()
            />
        </div>
    )
}
