use std::collections::HashMap;

use leptos::{leptos_dom::logging::console_log, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MultiEntryData {
    pub id: usize,
    pub data: HashMap<String, ValueType>,
}

impl MultiEntryData {
    pub fn new() -> Self {
        Self {
            id: 0,
            data: HashMap::new(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ValueType {
    String(String),
    Number(i32),
}

impl ValueType {
    pub fn as_string(&self) -> String {
        match self {
            ValueType::String(s) => s.clone(),
            ValueType::Number(n) => n.to_string(),
        }
    }
}

/// Represents a single entry in a MultiEntry component.
/// These are displayed in a list, and function a little bit like a dropdown.
/// When clicked, they expand to show all the inputs, which can be edited.
#[component]
pub fn Entry(
    /// The data for this entry.
    #[prop()]
    entryData: MultiEntryData,
    /// The main display member for this entry.
    #[prop()]
    nameMember: String,
    /// The secondary display member for this entry.
    #[prop()]
    infoMember: String,
) -> impl IntoView {
    let nameData = entryData.data.get(&nameMember).unwrap();
    let infoData = entryData.data.get(&infoMember).unwrap();

    view! {
        <div>
            <p>{nameData.as_string()}</p>
            <p>{infoData.as_string()}</p>
        </div>
    }
}

#[component]
pub fn MultiEntry(
    /// the main display member for each entry object.
    #[prop()]
    nameMember: String,
    /// The secondary display member for each entry object.
    #[prop()]
    infoMember: String,
) -> impl IntoView {
    let entries: Signal<Vec<MultiEntryData>> = Signal::from(vec![]);

    let nameKey = nameMember.clone();
    let infoKey = infoMember.clone();

    let create_entry = move |_| {
        console_log("Found button click");
        let mut new_entry = MultiEntryData::new();
        new_entry.id = entries.get().len() as usize;
        new_entry
            .data
            .insert(nameKey.clone(), ValueType::String(String::from("Testing")));
        new_entry
            .data
            .insert(infoKey.clone(), ValueType::Number(new_entry.id as i32));
        entries.get().push(new_entry);
        console_log(format!("{:?}", entries.get()).as_str());
    };

    view! {
        <div>
            // Entries will go here for right now.
            <For
                each = move || entries.get()
                key = |entry: &MultiEntryData| entry.id
                children = move |entry| view! {
                    <Entry entryData=entry.clone() nameMember=nameMember.clone() infoMember=infoMember.clone()/>
                }
            />
        </div>
            <button on:click=create_entry>
            Add Entry
        </button>
    }
}
