#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{Client, error::ProvideErrorMetadata};

#[cfg(feature = "ssr")]
use serde_dynamo::{from_item, to_item};

use chrono::{DateTime, Utc};
use leptos::either::EitherOf3;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use leptos::logging::log;
use leptos_router::params::Params;
use leptos_router::hooks::{use_navigate, use_params, use_query};
use leptos::Params;
use traits::{AsReactive, ReactiveCapture};
use crate::common::{ExpandableInfo, ValueType};
use crate::components::{ActionButton, Banner, DashboardButton, OutlinedTextField, Panel, Row, Select};
use std::collections::HashMap;

#[derive(Params, PartialEq)]
struct LoanerParams {
    form_name: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoanerReturnOutput {
    /// The subject ID of the student entry.
    subject: String,
    /// The first name of the student that is borrowing.
    first_name: String,
    /// The last name of the student that is borrowing.
    last_name: String,
}

#[server]
async fn get_loaners_return_list() -> Result<Vec<LoanerReturnOutput>, ServerFnError> {
    // We want to query the database for all loaners, but we only want the name and
    // date of borrowing. This will be cheaper than querying the entire table.
    let dbclient = Client::new(&aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await);

    log!("Getting loaners from database.");

    match dbclient
        .scan()
        .table_name("loaner-info")
        .projection_expression("subject, first_name, last_name")
        .send()
        .await {
        Ok(output) => {
            if let Some(items) = output.items {
                Ok(items.into_iter().map(|item| {
                    let expandable: ExpandableInfo = from_item(item).unwrap();
                    LoanerReturnOutput {
                        first_name: expandable.data.get("first_name").unwrap().as_string().unwrap_or_default().unwrap_or_default(),
                        last_name: expandable.data.get("last_name").unwrap().as_string().unwrap_or_default().unwrap_or_default(),
                        subject: expandable.subject
                    }
                }).collect())
            } else {
                Ok(vec![])
            }
        },
        Err(err) => {
            log!("{:?}", err);
            let msg = err.message().unwrap_or("Unknown error");
            Err(ServerFnError::new(msg))
        },
    }
}

#[server]
async fn create_borrow_entry(input: ExpandableInfo) -> Result<(), ServerFnError> {
    let mut input = input;
    let dbclient = Client::new(&aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await);
    
    loop {
        let item: serde_dynamo::Item = to_item(&input)?;
        match dbclient
            .put_item()
            .table_name("loaner-info")
            .set_item(Some(HashMap::from(item.clone())))
            .condition_expression("attribute_not_exists(subject)")
            .send()
            .await
        {
            Ok(_) => return Ok(()),
            Err(err) => {
                log!("{:?}", err);
                match err.code() {
                    Some("ConditionalCheckFailedException") => {
                        // Retry database write until we have a good subject ID.
                        log!("Failed conditional check, forcing new subject ID.");
                        input.subject = uuid::Uuid::new_v4().to_string();
                        continue
                    },
                    _ => {
                        let msg = err.message().unwrap_or("An unknown error occurred.");
                        return Err(ServerFnError::new(msg));
                    }
                };
            }
        }
    }
}

/// # Loaner Page
///
/// This page is going to temporarily piggy-back on the general scholarship app, at least until
/// we spin off the components used here into a new standalone crate.
///
/// The loaner app handles management of Chromebook loaners to students. It was originally hosted
/// using Google Apps Script, but has recently been broken by some unknown API error. I've chosen
/// to rewrite this into a single new application, which we will host in AWS with a real database.
///
/// It should use all the same components as the scholarship loaner, maybe with a few extra
/// alterations. The page will only use a single database table, and it will only have a few very
/// basic APIs.
///
/// # Design
///
/// The page should have two sections - one for logging a loaner that's being taken out (borrowing)
/// and another for a loaner that's being brought back (returning). As of right now, we have two
/// separate buttons that handle these functions, and they slide open a panel at the bottom of the
/// screen. The borrowing menu has a single, simple form, and the returning side has a list of
/// buttons that allow students to choose their name and remove it from the list.
///
/// # New features
///
/// I believe that with a new workflow and an overhauled server with real data management, we should
/// create a true admin page for this, simply to be able to survey and override/remove information
/// from the lists.
#[component]
pub fn LoanerPage() -> impl IntoView {
    // Check URL params. Depending on what string is visible here, we'll display a different form.
    let params = use_params::<LoanerParams>();
    let form_passed = move || {
        params.read()
            .as_ref()
            .ok()
            .and_then(|params| params.form_name.clone())
            .unwrap_or_default()
    };
    
    let form_view = move || {
        match form_passed().as_str() {
            "borrowing" => EitherOf3::A(LoanerBorrowForm),
            "returning" => EitherOf3::B(LoanerReturnForm),
            _ => EitherOf3::C("Choose something from the left.".into_view())
        }
    };

    // We'll create the page here - the sidebar contains all the buttons, while the form
    // container either shows forms or shows placeholder text.
    view! {
        <Banner
            title="Chromebook Loaners"
            logo="/PHS_Stacked_Acronym.png"
            path="/loaners"
        />
        <div class="flex m-5">
            <div class="flex flex-1 flex-row gap-3">
                <div id="sidebar" class="flex flex-1 flex-col gap-2">
                    <DashboardButton
                        title="Borrowing"
                        description="Select to take out a loaner."
                        icon="/Edit.png"
                        path="/loaners/borrowing"
                    />
                    <DashboardButton
                        title="Returning"
                        description="Select to return a loaner."
                        icon="/Person_Black.png"
                        path="/loaners/returning"
                    />
                </div>
                <div class="flex flex-2">
                    <Panel>{form_view}</Panel>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn LoanerBorrowForm() -> impl IntoView {
    // Register server actions.
    let create_entry_action = ServerAction::<CreateBorrowEntry>::new();
    let navigate = use_navigate();

    let temp = ExpandableInfo::new("test");
    let temp_reactive = temp.as_reactive();
    let elements_disabled = RwSignal::new(false);
    
    view! {
        <Row>
            <OutlinedTextField 
                label="First Name:"
                placeholder="John"
                disabled=elements_disabled
                data_member="first_name"
                data_map=temp_reactive.data
            />
            <OutlinedTextField 
                label="Last Name:"
                placeholder="Smith"
                disabled=elements_disabled
                data_member="last_name"
                data_map=temp_reactive.data
            />
        </Row>
        <Row>
            <OutlinedTextField 
                label="Region 15 Email:"
                placeholder="example@region15.org"
                disabled=elements_disabled
                data_member="email"
                data_map=temp_reactive.data
            />
        </Row>
        <Row>
            <Select
                label="Collateral:"
                value_list=vec![
                    "Phone".to_string(),
                    "Earbuds/Headphones".to_string(),
                    "Keys".to_string(),
                    "Wallet".to_string(),
                    "Laptop".to_string(),
                ]
                disabled=elements_disabled
                data_member="collateral"
                data_map=temp_reactive.data
            />
        </Row>
        <Row>
            <Select 
                label="Loan Taken:"
                value_list=vec![
                    "Chromebook".to_string(),
                    "Charger".to_string(),
                    "Headphones".to_string(),
                ]
                disabled=elements_disabled
                data_member="loan"
                data_map=temp_reactive.data
            />
        </Row>
        <Row>
            <ActionButton
                on:click=move |_| {
                    let captured = temp_reactive.capture();
                    log!("Loaner record taken.");
                    log!("{:?}", captured);
                    create_entry_action.dispatch(CreateBorrowEntry {
                        input: captured
                    });
                    navigate("/loaners", Default::default());
                }
            >"Submit"</ActionButton>
        </Row>
    }
}

#[component]
pub fn LoanerReturnForm() -> impl IntoView {
    // Register server resources
    let return_list_resource: Resource<Vec<LoanerReturnOutput>> = Resource::new(
        move || (),
        async |_| get_loaners_return_list().await.unwrap_or_else(|e| {
            log!("There was an error getting the loaner list: {}", e);
            vec![]
        })
    );
    
    view! {
        <Suspense fallback=move || "Loading...".into_view()>
            <div class="flex flex-col gap-2">
                {move || return_list_resource.get()
                    .map(move |loaner_list| {
                        loaner_list.into_iter().map(move |student| {
                            view! {
                                <span>{student.first_name}" "{student.last_name}</span>
                            }
                        }).collect_view()
                    })
                }
            </div>
        </Suspense>
    }
}
