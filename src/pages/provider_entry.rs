use leptos::prelude::*;
use crate::components::OutlinedTextField;

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
    )
}
