use std::collections::HashMap;
use std::sync::Arc;
use leptos::either::either;
use crate::components::{ActionButton, OutlinedTextField};
use leptos::prelude::*;
use crate::common::InputType;
use crate::components::ValueType;
use super::{multi_entry_data::*, multi_entry_member::*};

fn render_entry_component(entry_data: MultiEntryData, member_info: MultiEntryMember, label: String) -> impl IntoView {
    // Parse the multientry members into the correct components
    let data = entry_data.data.get(&member_info.member_name);
    let reactive = RwSignal::new(data.unwrap().clone());
    let temp = either!(member_info.input_type,
        InputType::Text => view! {
            <OutlinedTextField
                label=member_info.display_name
                data_member=member_info.member_name
                data_map=RwSignal::new(HashMap::default())
                placeholder="testing"
                value=reactive
            />
        },
        _ => view! {}
    );
    
    temp.into_view()
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
    /// The schema for all inputs that should be used, and what `MultiEntryMember` info is associated
    /// with them.
    #[prop()]
    schema: Arc<Vec<MultiEntryMember>>
) -> impl IntoView {
    let name_data = entry_data.data.get(&name_member.get().member_name).unwrap();
    let info_data = entry_data.data.get(&info_member.get().member_name).unwrap();

    let schema = Arc::clone(&schema);
    
    view! {
        // This div MUST have the same spacing CSS as the second-level header div
        <div class="flex rounded-sm transition-shadow shadow-sm hover:shadow-lg/30 p-3 m-1">
            <span class="flex-1">{name_data.to_string()}</span>
            <span class="flex-1">{info_data.to_string()}</span>
        </div>
        {schema
            .iter()
            .map(|input_type| {
                let data = entry_data.clone();
                let input = input_type.clone();
                view! {
                    // <div>{render_entry_component(data, input, "test".to_owned())}</div>
                }
            }).collect::<Vec<_>>()
        }
    }
}

/// Allows users to enter more open-ended information, and generates a `Vec<MultiDataEntry>` filled
/// with the user's entered information. The schema for this data must be defined by this component
/// such that it is constructable using a series of `String` keys, where each key stores a single
/// `ValueType` enum.
#[component]
pub fn MultiEntry(
    /// The list of entries to use.
    #[prop()]
    entries: RwSignal<Vec<MultiEntryData>>,
    /// The main display member for each entry object.
    #[prop()]
    name_member: Signal<MultiEntryMember>,
    /// The secondary display member for each entry object.
    #[prop()]
    info_member: Signal<MultiEntryMember>,
    /// The schema of the components. Determines what kind of data is stored and where.
    /// The `name_member` and `info_member` should be included in this list.
    #[prop(optional)]
    schema: Vec<MultiEntryMember>
) -> impl IntoView {
    // Creates an entry in the given entries list.
    let create_entry = move |_| {
        let mut new_entry = MultiEntryData::new();
        new_entry.data.insert(
            name_member.get().member_name,
            ValueType::String(Some(String::from("Testing"))),
        );
        new_entry.data.insert(
            info_member.get().member_name,
            ValueType::String(Some(new_entry.id.to_string())),
        );
        entries.update(|entry_vec| entry_vec.push(new_entry));
    };
    
    let rc_schema = Arc::new(schema);

    view! {
        <div class="flex-auto p-2 m-5">
            // Header section
            <div class="rounded-md bg-red-700">
                // This div MUST have the same spacing CSS as the main entry div
                <div class="flex bg-inherit p-3 m-1">
                    <span class="flex-1 text-white font-bold">
                        {name_member.get().display_name}
                    </span>
                    <span class="flex-1 text-white font-bold">
                        {info_member.get().display_name}
                    </span>
                </div>
            </div>
            // Entry section
            <div>
                <For
                    each=move || entries.get()
                    key=|entry: &MultiEntryData| entry.id
                    children=move |entry| {
                        let schema = Arc::clone(&rc_schema);
                        view! {
                            <Entry
                                entry_data=entry.clone()
                                name_member=name_member
                                info_member=info_member
                                schema=schema
                            />
                        }
                    }
                />
            </div>
        </div>
        <ActionButton on:click=create_entry>Add Entry</ActionButton>
    }
}
