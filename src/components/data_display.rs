use crate::common::{SchemaContainerStyle, SchemaHeaderStyle, SchemaNode, SchemaType, ValueType};
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn DataDisplay(
    /// The schema that this component should use to display the given data.
    #[prop(into)]
    schema: Signal<SchemaNode>,
    /// The starting key in the data map.
    #[prop(into, optional)]
    data_member: Signal<String>,
    /// A `HashMap` containing all the data that should be displayed.
    #[prop(into)]
    data_map: Signal<HashMap<String, ValueType>>,
) -> impl IntoView {
    // The schema should contain information about the header type and styling, along with the
    // information display. This is rendered as part of the DataDisplayWrapper, which is simply a
    // wrapper around the information to be displayed.

    // The children are all SchemaNodes as well, meaning that this component needs to call itself
    // for each of these children. They will of course contain all the same information that we
    // expect here.

    // The actual displayed information is controlled through a DataDisplayView, which will accept
    // settings for how the information is parsed. For example, rendering a list may be done by
    // a string join operation, or it may be a list of maps, each of which needs to be displayed with
    // a specific set of children. The SchemaNode must be able to handle this, and these options
    // should be given to the DataDisplayView somehow.

    // As the display_value here is actually an Option<ValueType>, we want to be able to handle cases
    // where the display is given no values. In this case, the SchemaNode needs to specify whether
    // a missing value is expected or an error.

    let display_value = Memo::new(move |_| {
        let member = data_member.get();
        if member.is_empty() {
            Some(ValueType::Map(Some(data_map.get())))
        } else {
            data_map.get().get(&member).cloned()
        }
    });

    view! {
        <DataDisplayWrapper
            container_style=schema.get().container_style
            header_style=schema.get().header_style
            header_text=schema.get().header
        >
            <DataDisplayView display_data=display_value schema=schema />
        </DataDisplayWrapper>
    }
}

#[component]
fn DataDisplayWrapper(
    #[prop(into)] container_style: SchemaContainerStyle,
    #[prop(into)] header_style: SchemaHeaderStyle,
    #[prop(into)] header_text: Signal<String>,
    children: Children,
) -> impl IntoView {
    // This component takes a wrapper style and displays the "chrome" of the interface. It has two
    // options: Text Headers or Capsules. This will then render a DataDisplayHeader, which handles
    // the styling for the text inside the header specifically.

    // The capsule style is a colored header and a rounded container.
    // The text header style is an invisible container.

    let expanded = RwSignal::new(true);

    match container_style {
        SchemaContainerStyle::Header => {
            // Creates a simple container that only has a header and a series of children.
            // The spacing between the header and children is controlled by the header style.
            // This type of container is not collapsable at the moment.
            view! {
                <div class="flex flex-col w-full text-left">
                    <div class="self-start w-full">
                        <DataDisplayHeader header_text=header_text header_style=header_style />
                        <div class="p-2 flex flex-col gap-1">{children()}</div>
                    </div>
                </div>
            }
            .into_any()
        }
        SchemaContainerStyle::Capsule => {
            // Creates a capsule container - it is collapsable, with rounded corners and a colored
            // header.
            view! {
                <div class="self-stretch rounded-lg shadow-lg flex flex-col w-full text-center">
                    <div
                        class="p-2 px-3 rounded-t-lg cursor-pointer bg-red-800 text-white"
                        class=(["rounded-lg"], move || !expanded.get())
                        on:click=move |_| expanded.update(|b| *b = !*b)
                    >
                        <DataDisplayHeader header_text=header_text header_style=header_style />
                    </div>
                    <div
                        class="grid transition-all ease-in-out"
                        style:grid-template-rows=move || if expanded.get() { "1fr" } else { "0fr" }
                    >
                        <div class="min-h-0 overflow-hidden">
                            <div class="p-2 flex flex-col gap-1 flex-1">{children()}</div>
                        </div>
                    </div>
                </div>
            }
            .into_any()
        }
    }
}

#[component]
fn DataDisplayHeader(
    #[prop(into)] header_style: SchemaHeaderStyle,
    #[prop(into)] header_text: Signal<String>,
) -> impl IntoView {
    // This component will take a header style and displays the corresponding text. It has a few
    // options, all of which are defined in the HeaderStyle enum.

    match header_style {
        SchemaHeaderStyle::MainHeader => {
            view! { <div class="text-3xl font-bold my-2">{header_text}</div> }.into_any()
        }
        SchemaHeaderStyle::SubsectionHeader => {
            view! { <div class="text-xl font-bold">{header_text}</div> }.into_any()
        }
        SchemaHeaderStyle::Bold => view! { <div class="font-bold">{header_text}</div> }.into_any(),
        SchemaHeaderStyle::None => view! {}.into_any(),
    }
}

#[component]
fn DataDisplayView(
    #[prop(into)] display_data: Signal<Option<ValueType>>,
    #[prop(into)] schema: Signal<SchemaNode>,
) -> impl IntoView {
    // This is the part of the data display that does most of the work. It will take the values
    // found in the DataDisplay component and perform the logic that actually displays the
    // information. This component is only displayed if there is Some(value) in the DataDisplay.

    // In the case that this is displaying a Map or List, it will recursively call the DataDisplay
    // component.

    // If this is displaying a PrimitiveList, it will flatten and join all the values into a single
    // string, then display that string.

    let data_type = schema.get().data_type;

    // Matches the specific data type. Displays errors if the given display data does not match.
    match data_type {
        SchemaType::String => {
            let value = Signal::derive(move || {
                display_data
                    .get()
                    .map(|v| v.as_string().ok().flatten())
                    .flatten()
                    .unwrap_or("N/A".to_string())
            });
            view! { <div>{value}</div> }.into_any()
        }
        SchemaType::Number => {
            let value = Signal::derive(move || {
                display_data
                    .get()
                    .map(|v| v.as_number().ok().flatten())
                    .flatten()
                    .unwrap_or("N/A".to_string())
            });
            view! { <div>{value}</div> }.into_any()
        }
        SchemaType::PrimitiveList => {
            let value = Signal::derive(move || {
                display_data
                    .get()
                    .map(|v| v.as_list().ok().flatten())
                    .flatten()
                    .unwrap_or_default()
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            });
            view! { <div>{value}</div> }.into_any()
        }
        SchemaType::MapList => {
            let list = Signal::derive(move || {
                display_data
                    .get()
                    .map(|v| v.as_list().ok().flatten())
                    .flatten()
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|v| v.as_map().ok().flatten().clone())
                    .collect::<Vec<HashMap<String, ValueType>>>()
            });
            let primary_key = StoredValue::new(schema.get().primary_key.unwrap_or_default());
            view! {
                <Show
                    when=move || !list.get().is_empty()
                    fallback=|| view! { <div class="w-full text-center">"List is empty."</div> }
                >
                    <For
                        each=move || list.get()
                        key=move |map| {
                            map.get(&primary_key.get_value())
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        }
                        children=move |map| {
                            view! {
                                <DataDisplay
                                    schema=*schema.get().item_template.unwrap_or_default()
                                    data_map=map
                                />
                            }
                        }
                    />
                </Show>
            }
            .into_any()
        }
        SchemaType::Map => {
            let map = Signal::derive(move || {
                display_data
                    .get()
                    .map(|v| v.as_map().ok().flatten())
                    .flatten()
            });
            view! {
                <Show
                    when=move || map.get().is_some()
                    fallback=|| view! { <div class="text-red-500">"Map was not found."</div> }
                >
                    <For
                        each=move || schema.get().children
                        key=|(key, _)| key.clone()
                        children=move |(key, schema)| {
                            view! {
                                <DataDisplay
                                    schema=schema
                                    data_member=key
                                    data_map=Signal::derive(move || map.get().unwrap_or_default())
                                />
                            }
                        }
                    />
                </Show>
            }
            .into_any()
        }
    }
}
