use std::collections::HashMap;

use crate::components::ActionButton;
use leptos::{leptos_dom::logging::console_log, prelude::*};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MultiEntryData {
    pub id: Uuid,
    pub data: HashMap<String, ValueType>,
}

impl MultiEntryData {
    /// Creates a new MultiEntryData struct.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            data: HashMap::new(),
        }
    }
}

/// Contains all information required to display an entry in the MultiEntry component.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MultiEntryMember {
    /// The name of the member, used for display only.
    pub display_name: String,
    /// The actual name of the member, used to get the data from an Entry struct.
    pub member_name: String,
}

impl MultiEntryMember {
    /// Creates a new MultiEntryMember struct.
    pub fn new() -> Self {
        Self {
            display_name: String::new(),
            member_name: String::new(),
        }
    }

    /// Creates a new MultiEntryMember struct with the specified information.
    pub fn from(display_name: String, member_name: String) -> Self {
        Self {
            display_name,
            member_name,
        }
    }

    /// Creates a new MultiEntryMember struct with the specified information. Uses string slices.
    pub fn from_str(display_name: &str, member_name: &str) -> Self {
        Self {
            display_name: display_name.into(),
            member_name: member_name.into(),
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
    entry_data: MultiEntryData,
    /// The main display member for this entry.
    #[prop()]
    name_member: Signal<MultiEntryMember>,
    /// The secondary display member for this entry.
    #[prop()]
    info_member: Signal<MultiEntryMember>,
) -> impl IntoView {
    let nameData = entry_data.data.get(&name_member.get().member_name).unwrap();
    let infoData = entry_data.data.get(&info_member.get().member_name).unwrap();

    view! {
        // This div MUST have the same spacing CSS as the second-level header div
        <div class="flex rounded-sm transition-shadow shadow-sm hover:shadow-lg/30 p-3 m-1">
            <span class="flex-1">{nameData.as_string()}</span>
            <span class="flex-1">{infoData.as_string()}</span>
        </div>
    }
}

#[component]
pub fn MultiEntry(
    /// The list of entries to use.
    #[prop()]
    entries: RwSignal<Vec<MultiEntryData>>,
    /// the main display member for each entry object.
    #[prop()]
    name_member: Signal<MultiEntryMember>,
    /// The secondary display member for each entry object.
    #[prop()]
    info_member: Signal<MultiEntryMember>,
) -> impl IntoView {
    // Creates an entry in the given entries list.
    let create_entry = move |_| {
        let mut new_entry = MultiEntryData::new();
        new_entry.data.insert(
            name_member.get().member_name,
            ValueType::String(String::from("Testing")),
        );
        new_entry.data.insert(
            info_member.get().member_name,
            ValueType::String(new_entry.id.to_string()),
        );
        entries.update(|entry_vec| entry_vec.push(new_entry));
    };

    view! {
        <div class="flex-auto p-2 m-5">
            // Header section
            <div class="rounded-md bg-red-700">
                // This div MUST have the same spacing CSS as the main entry div
                <div class="flex bg-inherit p-3 m-1">
                    <span class="flex-1 text-white font-bold">{name_member.get().display_name}</span>
                    <span class="flex-1 text-white font-bold">{info_member.get().display_name}</span>
                </div>
            </div>
            // Entry section
            <div>
                <For
                    each = move || entries.get()
                    key = |entry: &MultiEntryData| entry.id
                    children = move |entry| view! {
                        <Entry
                            entry_data=entry.clone()
                            name_member=name_member
                            info_member=info_member
                        />
                    }
                />
            </div>
        </div>
        <ActionButton on:click=create_entry>
            Add Entry
        </ActionButton>
    }
}
