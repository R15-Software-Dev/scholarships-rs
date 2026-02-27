#[cfg(feature = "ssr")]
mod imports {
    pub use super::super::{COMPARISONS_TABLE, PROVIDER_CONTACT_TABLE, SCHOLARSHIPS_TABLE};
    pub use crate::common::ValueType;
    pub use crate::utils::server::create_dynamo_client;
    pub use aws_sdk_dynamodb::Client;
    pub use aws_sdk_dynamodb::error::ProvideErrorMetadata;
    pub use indexmap::IndexMap;
    pub use std::collections::HashMap;
}

use crate::common::ExpandableInfo;
use leptos::prelude::*;

#[server]
pub async fn get_scholarship_csv() -> Result<Vec<u8>, ServerFnError> {
    use imports::*;
    use leptos::logging::debug_log;

    #[derive(serde::Serialize)]
    struct ResolvedScholarshipInfo {
        contact_email: String,
        first_name: String,
        last_name: String,
        scholarship_name: String,
        sports_requirements: String,
        gpa_requirements: String,
        major_requirements: String,
        additional_requirements: String,
        community_requirements: String,
        residency_requirements: String,
    }

    // Get the scholarships and comparisons
    let client = create_dynamo_client().await;

    let scan_table = async |client: &Client, table_name| {
        client
            .scan()
            .table_name(table_name)
            .send()
            .await
            .map(|output| output.items)
            .ok()
            .flatten()
            .unwrap_or_default()
            .into_iter()
            .map(|item| {
                serde_dynamo::from_item(item).unwrap_or(ExpandableInfo::new("".to_string()))
                // item.iter()
                //     .map(|(k, v)| (k.to_owned(), ValueType::from(v)))
                //     .collect::<HashMap<String, ValueType>>()
            })
            .collect::<Vec<ExpandableInfo>>()
    };

    let scan_table_hash_map = async |client: &Client, table_name| {
        client
            .scan()
            .table_name(table_name)
            .send()
            .await
            .map(|output| output.items)
            .ok()
            .flatten()
            .unwrap_or_default()
            .into_iter()
            .map(|item| {
                // serde_dynamo::from_item(item).unwrap_or(ExpandableInfo::new("".to_string()))
                item.iter()
                    .map(|(k, v)| (k.to_owned(), ValueType::from(v)))
                    .collect::<HashMap<String, ValueType>>()
            })
            .collect::<Vec<HashMap<String, ValueType>>>()
    };

    let master_scholarships = scan_table(&client, SCHOLARSHIPS_TABLE).await;
    let master_contacts = scan_table_hash_map(&client, PROVIDER_CONTACT_TABLE).await;
    let master_relations = scan_table_hash_map(&client, COMPARISONS_TABLE).await;

    // We now want to map the relations to the scholarships, as well as the contacts.
    // This should take the information from the current scholarship, then relate the IDs of the
    // requirements in the scholarship to the IDs of the relations in the master list.
    let resolved_scholarships = master_scholarships
        .into_iter()
        .map(|scholarship| {
            // debug_log!("Scholarship: {:?}", scholarship);
            // We want the requirements IDs and the provider ID.
            let requirements_map = scholarship
                .data
                .get("requirements")
                .unwrap_or(&ValueType::Map(None))
                .as_map()
                .ok()
                .flatten()
                .unwrap_or_default();

            let provider_id = scholarship
                .data
                .get("provider_id")
                .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                .unwrap_or_default();

            // Search for the provider ID in the master list.
            let contact_info = master_contacts
                .iter()
                .find(|contact| {
                    let subject = contact
                        .get("subject")
                        .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                        .unwrap_or_default();

                    debug_log!("Comparing {:?} and {:?}", subject, provider_id);

                    subject == provider_id
                })
                .cloned()
                .unwrap_or_default();

            // Resolve the requirements - this should resolve as another category-separated map,
            // but with requirement display text instead of IDs.
            let resolved_requirements = requirements_map
                .iter()
                .map(|(category, selected_val)| {
                    let selected_list = selected_val.as_list().ok().flatten().unwrap_or_default();

                    let resolved_names = selected_list
                        .into_iter()
                        .map(|id_val| {
                            let id = id_val.as_string().ok().flatten().unwrap_or_default();

                            let resolved_relation = master_relations
                                .iter()
                                .find(|item| {
                                    let comp_id = item
                                        .get("id")
                                        .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                                        .unwrap_or_default();

                                    comp_id == id
                                })
                                .cloned()
                                .map(|v| {
                                    v.get("display_text").map(|text_opt| {
                                        text_opt.as_string().ok().flatten().unwrap_or_default()
                                    })
                                })
                                .flatten()
                                .unwrap_or_default();

                            resolved_relation
                        })
                        .collect::<Vec<String>>();

                    (category.clone(), resolved_names)
                })
                .collect::<IndexMap<String, Vec<String>>>();
            debug_log!("Resolved requirements: {:?}", resolved_requirements);

            let contact_email = contact_info
                .get("contact_email")
                .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                .unwrap_or_default();

            let first_name = contact_info
                .get("first_name")
                .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                .unwrap_or_default();

            let last_name = contact_info
                .get("last_name")
                .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                .unwrap_or_default();

            let scholarship_name = scholarship
                .data
                .get("name")
                .map(|v| v.as_string().ok().flatten().unwrap_or_default())
                .unwrap_or_default();

            debug_log!(
                "{:?}, {:?}, {:?}, {:?}",
                scholarship_name,
                contact_email,
                first_name,
                last_name
            );

            // We want to create some structure that's got the correct information. Specifically,
            // we only need the provider contact info, the scholarship's name, and the requirements
            // that they chose in plain text.
            ResolvedScholarshipInfo {
                first_name,
                last_name,
                contact_email,
                scholarship_name,
                sports_requirements: resolved_requirements
                    .get("sports_participation")
                    .map(|list| list.join("; "))
                    .unwrap_or_default(),
                gpa_requirements: resolved_requirements
                    .get("gpa")
                    .map(|list| list.join("; "))
                    .unwrap_or_default(),
                major_requirements: resolved_requirements
                    .get("major")
                    .map(|list| list.join("; "))
                    .unwrap_or_default(),
                additional_requirements: resolved_requirements
                    .get("misc")
                    .map(|list| list.join("; "))
                    .unwrap_or_default(),
                community_requirements: resolved_requirements
                    .get("community_service")
                    .map(|list| list.join("; "))
                    .unwrap_or_default(),
                residency_requirements: resolved_requirements
                    .get("residency")
                    .map(|list| list.join("; "))
                    .unwrap_or_default(),
            }
        })
        .collect::<Vec<ResolvedScholarshipInfo>>();

    // Now that we have a series of scholarships, we need to serialize them into CSV and send the
    // file to the client.
    let mut writer = csv::Writer::from_writer(Vec::new());
    resolved_scholarships.iter().for_each(|scholarship| {
        _ = writer.serialize(scholarship);
    });
    writer.flush()?;

    let result = writer.into_inner()?;
    debug_log!("{:?}", String::from_utf8(result.clone()));

    Ok(result)
}
