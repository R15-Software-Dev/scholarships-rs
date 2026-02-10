use chrono::DateTime;
use crate::components::ActionButton;
use leptos::prelude::*;
use crate::common::{DateInfo, DateRange};
use crate::pages::api::{CreateDates, CreateTestComparisons};

fn get_important_dates() -> Vec<DateInfo> {
    vec! [
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Provider Forms Open".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-01-20T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-03-03T23:59:00-05:00").unwrap()
            ),
            description: "".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Student Forms Open".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-03-03T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-03-27T14:05:00-05:00").unwrap()
            ),
            description: "".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Scholarship Decision Window".to_string(),
            date: DateRange::Range(
                DateTime::parse_from_rfc3339("2026-04-10T07:15:00-05:00").unwrap(),
                DateTime::parse_from_rfc3339("2026-05-01T23:59:00-05:00").unwrap()
            ),
            description: "Start date may change if available sooner.".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "PHS Scholarship Committee".to_string(),
            date: DateRange::Single(
                DateTime::parse_from_rfc3339("2026-05-18T00:00:00-05:00").unwrap(),
            ),
            description: "Time TBD".to_string(),
        },
        DateInfo {
            id: uuid::Uuid::new_v4().to_string(),
            title: "Scholarship and Awards Night".to_string(),
            date: DateRange::Single(
                DateTime::parse_from_rfc3339("2026-06-09T00:00:00-05:00").unwrap(),
            ),
            description: "Time TBD".to_string(),
        },
    ]
}

#[server]
async fn create_scholarship_export() -> Result<Vec<u8>, ServerFnError> {
    // Exports olarship information - specifically the name of the scholarship, the name of the
    // provider, and the scholarship's requirements.
    // This means we'll need to get all the relations, all the scholarships, and all the contact info.
    use crate::utils::server::create_dynamo_client;
    use aws_sdk_dynamodb::client::Client;
    use crate::pages::api::{get_comparison_info, SCHOLARSHIPS_TABLE, PROVIDER_CONTACT_TABLE};
    use std::collections::HashMap;
    use crate::common::ValueType;

    let scan_table_items = async |client: &Client, table_name: &str| {
        client.scan()
            .table_name(table_name)
            .send()
            .await
            .ok().and_then(|res| res.items)
            .unwrap_or_default()
    };
    
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct ScholarshipInfo {
        name: String,
        provider_name: String,
        relations: Vec<String>,
    }
    
    let client = create_dynamo_client().await;
    
    let scholarships = dbg!(scan_table_items(&client, SCHOLARSHIPS_TABLE)
        .await
        .iter().map(|item| {
            item.into_iter()
                .map(|(k, v)| (k.clone(), ValueType::from(v)))
                .collect()
        }).collect::<Vec<HashMap<String, ValueType>>>()
    );
    let contacts = dbg!(
        scan_table_items(&client, PROVIDER_CONTACT_TABLE).await
            .iter().map(|item| {
                item.into_iter()
                    .map(|(k, v)| (k.clone(), ValueType::from(v)))
                    .collect()
            }).collect::<Vec<HashMap<String, ValueType>>>()
    );
    let db_relations = dbg!(
        get_comparison_info().await.unwrap_or_default()
    );
    
    // Build a new structure containing some of the scholarship's information.
    let resolved = scholarships.into_iter()
        .map(|scholarship| {
            // We just want the scholarship's name, the relations, and the provider's name.
            // In here, we're going to resolve all the scholarship's information to provide this in
            // a readable format.
            let name = scholarship.get("name")
                .unwrap_or(&ValueType::String(None))
                .to_string();
            
            // The list of relations - we're going to search the master list for each comparison's
            // name. It's structured as a map of lists, or a ValueType::Map(Option<HashMap<String, ValueType::List(Option<Vec<ValueType::String()>>)>>)
            let relations = scholarship.get("requirements")
                .unwrap_or(&ValueType::Map(None))
                .as_map().ok().flatten()
                .unwrap_or_default()  // We have our map of categories, now we need all the values inside each category.
                .iter()
                .map(|(_, list)| {
                    list.as_list().ok().flatten()
                        .unwrap_or_default()
                })
                .flatten()
                .collect::<Vec<ValueType>>()  // This is a Vec<ValueType::String>
                .iter().map(|val| val.clone().as_string().ok().flatten().unwrap_or_default())
                .collect::<Vec<String>>();
            
            // The provider ID, which we'll use to search for the provider's name.
            let provider_id = scholarship.get("provider_id")
                .unwrap_or(&ValueType::String(None))
                .to_string();
            
            // This is a mess, we'll clean it up later.
            let parsed_relations = relations
                .iter()
                .filter_map(|relation| {
                    db_relations.iter().find(|search| {
                        search.id == *relation
                    })
                        .and_then(|search| Some(search.display_text.clone()))
                })
                .collect::<Vec<String>>();
            
            ScholarshipInfo {
                name,
                provider_name: provider_id,
                relations: parsed_relations,
            }
        })
        .collect::<Vec<ScholarshipInfo>>();
    
    // TODO Convert into CSV and send to client. For now I think I'm just going to serialize to json and print it.
    leptos::logging::log!("Scholarships list: {:?}", resolved);
    
    Ok(Vec::new())
}

#[component]
pub fn AdminUtilsPage() -> impl IntoView {
    let create_comparisons = ServerAction::<CreateTestComparisons>::new();
    let create_dates = ServerAction::<CreateDates>::new();
    let create_export = ServerAction::<CreateScholarshipExport>::new();

    let on_click_comparisons = move |_| {
        create_comparisons.dispatch(CreateTestComparisons{});
    };

    let on_click_dates = move |_| {
        create_dates.dispatch(CreateDates{dates: get_important_dates()});
    };

    let on_click_export = move |_| {
        create_export.dispatch(CreateScholarshipExport{});
    };

    view! {
        <div class="mx-auto px-6">
            <div class="flex flex-col gap-4">
                <div class="self-centered text-lg mt-6">
                    "This utility page provides buttons to initialize the corresponding lists of information."
                </div>
                <ActionButton on:click=on_click_comparisons>"Create Comparisons"</ActionButton>
                <ActionButton on:click=on_click_dates>"Create Dates"</ActionButton>
                <ActionButton on:click=on_click_export>"Scholarship Export"</ActionButton>
            </div>
        </div>
    }.into_any()
}
