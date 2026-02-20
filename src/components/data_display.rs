use crate::common::{SchemaNode, SchemaType, ValueType};
use indexmap::IndexMap;
use leptos::logging::debug_log;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn ValueDisplay(
    #[prop(into)] schema: Signal<SchemaNode>,
    #[prop(into, optional)] data_member: Signal<String>,
    #[prop(into)] data_map: Signal<HashMap<String, ValueType>>,
) -> impl IntoView {
    let data_value = Memo::new(move |_| {
        if data_member.get().is_empty() {
            ValueType::Map(Some(data_map.get()))
        } else {
            data_map
                .get()
                .get(&data_member.get())
                .map(|v| v.clone())
                .unwrap_or_default()
        }
    });

    view! {
        {move || {
            match schema.get().data_type {
                SchemaType::String | SchemaType::Number => {
                    view! {
                        <PrimitiveDisplay
                            header=move || schema.get().header
                            value=data_value
                            nested=true
                        />
                    }
                        .into_any()
                }
                SchemaType::List => {
                    view! {
                        <ListDisplay
                            header=move || schema.get().header
                            value=data_value
                            item_template=move || {
                                schema
                                    .get()
                                    .item_template
                                    .unwrap_or(Box::from(SchemaNode::new(SchemaType::List)))
                            }
                            nested=true
                        />
                    }
                        .into_any()
                }
                SchemaType::Map => {
                    view! {
                        <MapDisplay
                            value=data_value
                            header=move || schema.get().header
                            schema=move || schema.get().children
                            nested=true
                        />
                    }
                        .into_any()
                }
            }
        }}
    }
}

#[component]
fn DisplayWrapper(
    #[prop(into, optional)] header: Signal<String>,
    #[prop(into, optional)] nested: Signal<bool>,
    children: Children,
) -> impl IntoView {
    // This component is the wrapper that is used in the rest of the display components.
    // It allows for simpler styling of the header and capsule, along with simple collapsable
    // functionality.

    let expanded = RwSignal::new(true);

    view! {
        <div class="m-3 rounded-lg shadow-lg flex flex-col">
            <div
                class="p-2 px-3 font-bold rounded-t-lg cursor-pointer"
                class=(["bg-gray-300"], move || !nested.get())
                class=(["text-white", "bg-red-900"], move || nested.get())
                class=(["rounded-lg"], move || !expanded.get())
                on:click=move |_| expanded.update(|b| *b = !*b)
            >
                {header}
            </div>
            <div
                class="grid transition-all ease-in-out"
                style:grid-template-rows=move || if expanded.get() { "1fr" } else { "0fr" }
            >
                <div class="min-h-0 overflow-hidden">
                    <div class="p-2">{children()}</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PrimitiveDisplay(
    #[prop(into)] header: Signal<String>,
    #[prop(into)] value: Signal<ValueType>,
    #[prop(into, optional)] nested: Signal<bool>,
) -> impl IntoView {
    let display_string = Memo::new(move |_| {
        let value = value.get();
        match value {
            ValueType::String(_) | ValueType::Number(_) => value.to_string(),
            _ => "Value is not a string.".to_string(),
        }
    });

    view! {
        <DisplayWrapper header=header nested=nested>
            <div>{display_string}</div>
        </DisplayWrapper>
    }
}

#[component]
fn ListDisplay(
    #[prop(into)] item_template: Signal<Box<SchemaNode>>,
    #[prop(into)] value: Signal<ValueType>,
    #[prop(into)] header: Signal<String>,
    #[prop(into, optional)] nested: Signal<bool>,
) -> impl IntoView {
    // Unwrap the value and check if it's the correct type. To be displayed with this component, it
    // MUST be a ValueType::List.
    let display_list = Memo::new(move |_| {
        let ValueType::List(Some(list)) = value.get() else {
            return Vec::new();
        };

        list.into_iter()
            .enumerate()
            .collect::<Vec<(usize, ValueType)>>()
    });

    let is_valid = Memo::new(move |_| matches!(value.get(), ValueType::List(_)));

    let item_header = Memo::new(move |_| item_template.get().header);

    view! {
        <DisplayWrapper header=header nested=nested>
            <For
                each=move || display_list.get()
                key=|(idx, _)| idx.clone()
                children=move |(idx, entry)| {
                    match item_template.get().data_type {
                        SchemaType::String | SchemaType::Number => {
                            view! { <PrimitiveDisplay value=entry header=item_header /> }.into_any()
                        }
                        SchemaType::List => {
                            view! {
                                <div>"Nested lists are not currently supported for display."</div>
                            }
                                .into_any()
                        }
                        SchemaType::Map => {
                            let map_header = format!("{} {}", item_template.get().header, idx + 1);
                            view! {
                                <MapDisplay
                                    value=entry
                                    header=map_header
                                    schema=move || item_template.get().children
                                />
                            }
                                .into_any()
                        }
                    }
                }
            />
        </DisplayWrapper>
    }
}

#[component]
fn MapDisplay(
    #[prop(into)] value: Signal<ValueType>,
    #[prop(into)] header: Signal<String>,
    #[prop(into)] schema: Signal<IndexMap<String, SchemaNode>>,
    #[prop(into, optional)] nested: Signal<bool>,
) -> impl IntoView {
    // Unwrap the value and check if it is the correct type. It must be a ValueType::Map.
    let display_map = Memo::new(move |_| {
        let ValueType::Map(Some(map)) = value.get() else {
            return HashMap::new();
        };

        map
    });

    Effect::new(move || {
        debug_log!("Current map values: {:?}", display_map.get());
    });

    let is_valid = Memo::new(move |_| matches!(value.get(), ValueType::Map(_)));

    view! {
        <DisplayWrapper header=header nested=nested>
            <For
                each=move || schema.get()
                key=|(k, _)| k.clone()
                children=move |(member, child_schema)| {
                    let display_value = Memo::new(move |_| {
                        display_map.get().get(&member).map(|v| v.clone()).unwrap_or_default()
                    });
                    match child_schema.data_type {
                        SchemaType::String | SchemaType::Number => {
                            view! {
                                <PrimitiveDisplay value=display_value header=child_schema.header />
                            }
                                .into_any()
                        }
                        SchemaType::List => {
                            let template = child_schema
                                .item_template
                                .unwrap_or(Box::from(SchemaNode::new(SchemaType::List)));

                            view! {
                                <ListDisplay
                                    value=display_value
                                    item_template=template
                                    header=child_schema.header
                                />
                            }
                                .into_any()
                        }
                        SchemaType::Map => {
                            view! {
                                <MapDisplay
                                    value=display_value
                                    schema=child_schema.children
                                    header=child_schema.header
                                />
                            }
                                .into_any()
                        }
                    }
                }
            />
        </DisplayWrapper>
    }
}
