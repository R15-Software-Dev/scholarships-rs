use crate::common::ValueType;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn ValueDisplay(
    #[prop(into)] data_member: Signal<String>,
    #[prop(into)] data_map: Signal<HashMap<String, ValueType>>,
    #[prop(into)] header: Signal<String>,
) -> impl IntoView {
    let display_value = Memo::new(move |_| {
        data_map
            .get()
            .get(&data_member.get())
            .unwrap_or(&ValueType::String(None))
            .clone()
    });

    // I've chosen to display all information - we're not going to worry about the depth of the
    // information, but we are going to provide a way to collapse information like maps and lists.
    // This means we need some capsule-like appearance for the information in order to facilitate
    // having a header and a series of information that's buried underneath. Also, we need to consider
    // how we determine what the header is because that's not really procedurally possible.

    // One possible way to handle this is a "schema." We would plan out the information beforehand
    // and pass through this schema struct, allowing us to limit what's displayed, relative to the
    // starting data_map. I'll write this out in a scratch file for now.

    view! {
        <div class="flex flex-1 flex-col">
            <div class="font-bold">{header}</div>
            <div>{move || display_value.get().to_string()}</div>
        </div>
    }
}

#[component]
pub fn MapDisplay() {}
