#[cfg(feature = "ssr")]
use crate::utils::server::create_dynamo_client;

#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::error::ProvideErrorMetadata;

#[cfg(feature = "ssr")]
use leptos::logging::log;

use crate::common::{
    ComparisonData, ComparisonType, MapListComparison, NumberComparison, NumberListComparison,
    TextComparison, TextListComparison, ValueType,
};
use std::collections::HashMap;
use leptos::prelude::ServerFnError;
use leptos::server;

#[cfg(feature = "ssr")]
static COMPARISONS_TABLE: &str = "leptos-comparison-test";

/// Helper function to create all sports comparisons. Only used to create initial
/// comparison data, and should be removed upon completion of a comparison editor page.
#[allow(unused)]
fn create_sports_comparisons() -> Vec<ComparisonData> {
    vec![
        ComparisonData::new(
            "sports_football",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Football".to_string())),
            "Sports Participation",
            "Football",
        ),
        ComparisonData::new(
            "sports_soccer",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Soccer".to_string())),
            "Sports Participation",
            "Soccer",
        ),
        ComparisonData::new(
            "sports_cheerleading",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Cheerleading".to_string())),
            "Sports Participation",
            "Cheerleading",
        ),
        ComparisonData::new(
            "sports_field_hockey",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Field Hockey".to_string())),
            "Sports Participation",
            "Field Hockey",
        ),
        ComparisonData::new(
            "sports_swimming",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Swimming".to_string())),
            "Sports Participation",
            "Swimming",
        ),
        ComparisonData::new(
            "sports_golf",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Golf".to_string())),
            "Sports Participation",
            "Golf",
        ),
        ComparisonData::new(
            "sports_basketball",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Basketball".to_string())),
            "Sports Participation",
            "Basketball",
        ),
        ComparisonData::new(
            "sports_track",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Track".to_string())),
            "Sports Participation",
            "Track",
        ),
        ComparisonData::new(
            "sports_gymnastics",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Gymnastics".to_string())),
            "Sports Participation",
            "Gymnastics",
        ),
        ComparisonData::new(
            "sports_ice_hockey",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Ice Hockey".to_string())),
            "Sports Participation",
            "Ice Hockey",
        ),
        ComparisonData::new(
            "sports_ski",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Ski".to_string())),
            "Sports Participation",
            "Ski",
        ),
        ComparisonData::new(
            "sports_wrestling",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Wrestling".to_string())),
            "Sports Participation",
            "Wrestling",
        ),
        ComparisonData::new(
            "sports_lacrosse",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Lacrosse".to_string())),
            "Sports Participation",
            "Lacrosse",
        ),
        ComparisonData::new(
            "sports_softball",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Softball".to_string())),
            "Sports Participation",
            "Softball",
        ),
        ComparisonData::new(
            "sports_tennis",
            "sports_participation",
            ComparisonType::MapList(MapListComparison::FlattenToTextList(
                "sport_name".to_string(),
                Box::new(TextListComparison::Contains),
            )),
            ValueType::String(Some("Tennis".to_string())),
            "Sports Participation",
            "Tennis",
        ),
    ]
}

fn create_major_comparison(
    id: impl Into<String>,
    display_text: impl Into<String>,
) -> ComparisonData {
    let display = display_text.into();
    ComparisonData::new(
        id,
        "major",
        ComparisonType::Text(TextComparison::Contains),
        ValueType::String(Some(display.clone())),
        "Majors",
        display,
    )
}

fn create_major_comparisons() -> Vec<ComparisonData> {
    let display_texts = vec![
        "Music",
        "Education",
        "Special Education",
        "Speech Pathology",
        "School Psychology",
        "School Counseling",
        "Occupational Therapy",
        "Physical Therapy",
        "Nursing",
        "Allied Health",
        "Fine/Performing Arts",
        "Writing/Communication",
        "History",
        "Government",
        "Political Science",
        "Social Work",
        "Sports Medicine",
        "Athletic Training",
        "Horticulture",
        "Conservation Studies",
        "Ecology",
        "Environmental Studies",
        "Urban Planning",
        "Landscaping",
        "Legal Studies",
        "Criminal Justice",
    ];

    let ids = display_texts
        .iter()
        .map(|text|
            format!("{}_{}", "major", text.to_lowercase().replace(" ", "_").replace("/", "_"))
        )
        .collect::<Vec<String>>();

    ids.iter().zip(display_texts).map(|(id, display_text)| {
        create_major_comparison(id, display_text)
    })
        .collect::<Vec<ComparisonData>>()
}

#[allow(unused)]
fn make_comp_list() -> Vec<ComparisonData> {
    let gpa_3 = ComparisonData::new(
        "gpa_3",
        "weighted_gpa",
        ComparisonType::Number(NumberComparison::GreaterThanOrEqual),
        ValueType::Number(Some(3.0.to_string())),
        "GPA Limits",
        "GPA >= 3.0",
    );

    let test_comp = ComparisonData::new(
        "math_sat_comp",
        "math_sat",
        ComparisonType::Number(NumberComparison::GreaterThanOrEqual),
        ValueType::Number(Some(650.to_string())),
        "Academic Information",
        "Math SAT Score > 650",
    );
    
    let service_hours_20 = ComparisonData::new(
        "service_hours_20",
        "community_involvement",
        ComparisonType::MapList(MapListComparison::FlattenToNumberList(
            "service_hours".to_string(),
            Box::new(NumberListComparison::Sum(Box::new(
                NumberComparison::GreaterThanOrEqual,
            ))),
        )),
        ValueType::Number(Some(20.to_string())),
        "Community Service",
        "20+ service hours",
    );

    let service_hours_25 = ComparisonData::new(
        "service_hours_25",
        "community_involvement",
        ComparisonType::MapList(MapListComparison::FlattenToNumberList(
            "service_hours".to_string(),
            Box::new(NumberListComparison::Sum(Box::new(
                NumberComparison::GreaterThanOrEqual,
            ))),
        )),
        ValueType::Number(Some(25.to_string())),
        "Community Service",
        "25+ service hours",
    );

    let service_hours_30 = ComparisonData::new(
        "service_hours_30",
        "community_involvement",
        ComparisonType::MapList(MapListComparison::FlattenToNumberList(
            "service_hours".to_string(),
            Box::new(NumberListComparison::Sum(Box::new(
                NumberComparison::GreaterThanOrEqual,
            ))),
        )),
        ValueType::Number(Some(30.to_string())),
        "Community Service",
        "30+ service hours",
    );

    let residency_southbury = ComparisonData::new(
        "residency_southbury",
        "town",
        ComparisonType::Text(TextComparison::Matches),
        ValueType::String(Some("Southbury".to_string())),
        "Residency",
        "Southbury",
    );

    let residency_middlebury = ComparisonData::new(
        "residency_middlebury",
        "town",
        ComparisonType::Text(TextComparison::Matches),
        ValueType::String(Some("Middlebury".to_string())),
        "Residency",
        "Middlebury",
    );

    let attended_bas = ComparisonData::new(
        "attended_bas",
        "attended_bas",
        ComparisonType::Text(TextComparison::Matches),
        ValueType::String(Some("Yes".to_string())),
        "Additional Eligibility Factors",
        "Attended BAS",
    );

    let midd_south = ComparisonData::new(
        "midd_south",
        "middsouth_church",
        ComparisonType::Text(TextComparison::Matches),
        ValueType::String(Some("Yes".to_string())),
        "Additional Eligibility Factors",
        "Member of Midd-South Church",
    );

    let family_military = ComparisonData::new(
        "family_military",
        "family_military_service",
        ComparisonType::Text(TextComparison::Matches),
        ValueType::String(Some("Yes".to_string())),
        "Additional Eligibility Factors",
        "Family with Military Service",
    );

    let mut sports_comps = create_sports_comparisons();
    let mut major_comps = create_major_comparisons();

    let mut comp_list = vec![
        gpa_3,
        test_comp,
        service_hours_20,
        service_hours_25,
        service_hours_30,
        residency_middlebury,
        residency_southbury,
        attended_bas,
        midd_south,
        family_military,
    ];

    comp_list.append(&mut sports_comps);
    comp_list.append(&mut major_comps);

    comp_list
}

#[server(CreateTestComparisons, endpoint = "/comparisons/create-test")]
pub async fn create_test_comparisons() -> Result<(), ServerFnError> {
    let client = create_dynamo_client().await;

    log!("Creating test comparisons");

    let comp_list = make_comp_list();

    for comparison in comp_list {
        if let Err(err) = client
            .put_item()
            .table_name(COMPARISONS_TABLE)
            .set_item(Some(serde_dynamo::to_item(&comparison)?))
            .send()
            .await
        {
            let msg = err.message().unwrap_or("An unknown error occurred");
            return Err(ServerFnError::new(msg));
        }
    }

    Ok(())
}

#[server]
pub async fn get_comparison_info() -> Result<Vec<ComparisonData>, ServerFnError> {
    let client = create_dynamo_client().await;

    log!("Getting all comparisons from the database");

    // Query the database for all comparisons. The client is only going to use the
    // id and display text, but we'll return the whole thing.
    match client
        .scan()
        .table_name(COMPARISONS_TABLE)
        .send()
        .await
    {
        Ok(output) => {
            if let Some(items) = output.items {
                log!("Found comparisons from API: {:?}", items);
                Ok(serde_dynamo::from_items(items)?)
            } else {
                Ok(vec![])
            }
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server]
pub async fn get_comparisons_categorized() -> Result<HashMap<String, Vec<ComparisonData>>, ServerFnError> {
    let client = create_dynamo_client().await;
    log!("Getting all comparisons from the database, by category.");

    match client
        .scan()
        .table_name(COMPARISONS_TABLE)
        .send()
        .await
    {
        Ok(output) => {
            if let Some(items) = output.items {
                log!("Found comparisons, categorizing.");

                let items: Vec<ComparisonData> = serde_dynamo::from_items(items)?;

                let categorized = items.iter().fold(
                    HashMap::<String, Vec<ComparisonData>>::new(), |mut map, comp| {
                    // Load each comparison into a map, then return that map.
                        map.entry(comp.category.clone())
                            .and_modify(|vec| vec.push(comp.clone()))
                            .or_insert(vec![comp.clone()]);

                        map
                    }
                );

                Ok(categorized)
            } else {
                Err(ServerFnError::new("Couldn't find any comparisons."))
            }
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred");
            Err(ServerFnError::new(msg))
        }
    }
}
