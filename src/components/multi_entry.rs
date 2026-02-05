use crate::common::{InputType, ValueType};
use crate::components::ActionButton;
use leptos::prelude::*;
use std::collections::HashMap;
use leptos::logging::log;
use uuid::Uuid;

// region Helper Functions

/// Gets the UUID from a given `HashMap<String, ValueType>`, wrapped in an `Option`.
fn get_uuid_from_map(map: &HashMap<String, ValueType>) -> Option<String> {
    map.get("uuid")?.as_string().unwrap_or(None)
}

/// Updates an Entry within the given `Vec`.
fn update_entry_list(list: &mut Vec<ValueType>, data: &HashMap<String, ValueType>) {
    if let Some(uuid) = get_uuid_from_map(&data) {
        if let Some(index) = list.iter().position(|x| {
            get_uuid_from_map(&x.as_map().unwrap().unwrap_or_default()) == Some(uuid.clone())
        }) {
            list[index] = ValueType::Map(Some(data.clone()));
        }
    }
}

/// Creates a new entry by creating a `ValueType::Map` with a unique ID at the key `uuid`.
/// This key can be found by using the `HashMap::get` function.
fn new_entry() -> ValueType {
    let mut entry_map = HashMap::new();
    entry_map.insert(
        "uuid".into(),
        ValueType::String(Some(Uuid::new_v4().to_string())),
    );
    ValueType::Map(Some(entry_map))
}

// endregion

/// # Multi-Entry Component
///
/// A rewrite of the `MultiEntry` component. Uses a single source of truth in the form of the `data_map`
/// prop. Creates and displays a series of `Entry` components that use their own `HashMap` signals.
/// Each of these `Entry` components runs a callback that alters the original `data_map`.
///
/// The `schema` prop determines what inputs are available to all `Entry` components created from
/// this component. For example, using a `schema` that contains an `InputType::Text` and an
/// `InputType::Select` means that all `Entry` components will render a text and dropdown input
/// component. The users shall be able to use these components to alter the data.
///
/// Example usage:
/// ```
/// let data_signal = RwSignal::new(HashMap::new());
/// view! {
///     <MultiEntryRewrite
///         data_member = "example"
///         data_map = data_signal
///         schema = vec![
///             InputType::Text("first_name", "First Name:", "John"),
///             InputType::Text("last_name", "Last Name:", "Smith")
///         ]
///     />
/// }
/// ```
///
/// In this example, any time a new `Entry` is created, it will contain two text fields that edit the
/// `first_name` and `last_name` keys within that `Entry`.
#[component]
pub fn MultiEntry(
    #[prop(into)] label: Signal<String>,
    #[prop(optional, into)] description: Signal<String>,
    /// The data member that contains the list of entries within the `data_map`.
    #[prop(into)]
    data_member: Signal<String>,
    /// The data map that contains all the information. Data is found within this map using the
    /// component's `data_member` input.
    #[prop()]
    data_map: RwSignal<HashMap<String, ValueType>>,
    /// The type and order of the inputs for each `Entry` component within this `MultiEntry` component.
    #[prop(optional, into)]
    schema: Signal<Vec<InputType>>,
) -> impl IntoView {
    // Make the values within the list reactive. Each of these values should be a ValueType::Map,
    // so we can pass these directly to the application.
    let data_list = Memo::new(move |_| {
        data_map.with(|map| {
            map.get(&data_member.get())
                .and_then(|v| v.as_list().ok().flatten())
                .unwrap_or_default()
        })
    });
    
    // This effect updates the data_map with any new values from the data_list, every time
    // the data_list is changed. This allows changes from within the Entry component to
    // bubble up to the data_map effectively.
    let update_parent_list = move |list| {
        log!("Updating parent list using child list {:?}", list);
        data_map.update(|map| {
            map.insert(data_member.get(), ValueType::List(Some(list)));
        });
    };

    let add_entry = move |_| data_map.update(|map| {
        log!("Adding new entry.");
        let mut list = data_list.get_untracked();
        
        list.push(new_entry());
        map.insert(data_member.get(), ValueType::List(Some(list)));
    });

    view! {
        <div class="flex flex-col flex-1">
            <div class="flex flex-col gap-2 p-2">
                <span class="font-bold">{label}</span>
                <Show when=move || !description.get().is_empty()>
                    <span>{description}</span>
                </Show>
            </div>
            <div class="flex flex-col gap-2 p-2">
                // Render the entries
                <Show
                    when=move || !data_list.get().is_empty()
                    fallback=|| {
                        view! { <div class="mx-auto">"You haven't added any entries yet."</div> }
                    }
                >
                    <For
                        each=move || data_list.get()
                        // Entries MUST be given a unique ID.
                        key=|entry| get_uuid_from_map(&entry.as_map().unwrap().unwrap())
                        children=move |mut entry_map| {
                            let map_signal = RwSignal::new(
                                entry_map.as_map().ok().flatten().unwrap_or_default(),
                            );
            
                            let unique_id = Memo::new(move |_| {
                                get_uuid_from_map(&map_signal.get()).unwrap_or_default()
                            });
            
                            Effect::new(move |_| {
                                let child_map = map_signal.get();
                                let mut list = data_list.get_untracked();
                                update_entry_list(&mut list, &child_map);
                                update_parent_list(list);
                            });

                            view! { <Entry data_map=map_signal schema=schema id=unique_id /> }
                        }
                    />
                </Show>
            </div>

            <ActionButton on:click=add_entry>"Add entry"</ActionButton>
        </div>
    }
}

/// # Entry Component
///
/// A single entry component. Should only ever be created from within a `MultiEntry` component.
/// Stores information within the `data_map` prop by using a series of data members that are found
/// within the `schema` list prop.
///
/// Example usage:
/// ```
/// // From within a MultiEntry component only
/// let data = RwSignal::new(HashMap::new());
/// view! {
///     <Entry
///         data_map = data
///         name_member = "first_name"
///         schema = vec![
///             InputType::Text("first_name", "First Name:", "John"),
///             InputType::Text("last_name", "Last Name:", "Smith")
///         ]
///     />
/// }
/// ```
///
/// In this example, the `Entry` component that is rendered will have two text inputs. Each of these
/// inputs will alter the `first_name` and `last_name` keys in the `data_map` respectively, and the
/// `Entry` component will run a callback to its parent `MultiEntry` component to update the `data_map`
/// accordingly.
#[component]
fn Entry(
    /// The data map that contains all information. Data is found within this map using the `member`
    /// props. This map should be unique to this entry; in other words, it should be contained and
    /// passed down from within the entry list in the parent `MultiEntry` component.
    #[prop()] data_map: RwSignal<HashMap<String, ValueType>>,
    /// The schema of this entry. Should contain all information about what inputs to use and what
    /// data members those inputs modify.
    #[prop(into)] schema: Signal<Vec<InputType>>,
    /// The unique ID for this entry.
    #[prop(into)] id: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col flex-1 p-2 rounded-sm transition-shadow shadow-sm hover:shadow-lg/30">
            {schema
                .get()
                .iter()
                .map(|input_type| 
                    input_type
                        .clone()
                        .into_view(data_map.clone(), id.get())
                )
                .collect_view()}
        </div>
    }
}
