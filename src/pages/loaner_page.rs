#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{Client, error::ProvideErrorMetadata, types::AttributeValue};

#[cfg(feature = "ssr")]
use serde_dynamo::{from_item, to_item};

use leptos::either::{Either, EitherOf3};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use leptos::logging::log;
use leptos_router::params::Params;
use leptos_router::hooks::{use_navigate, use_params};
use leptos::Params;
use traits::{AsReactive, ReactiveCapture};
use crate::common::{ExpandableInfo, ValueType};
use crate::components::{ActionButton, Banner, DashboardButton, OutlinedTextField, Panel, Row, Select};
use std::collections::HashMap;
use chrono::{FixedOffset, TimeZone};
use leptos::html::Dialog;

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
    /// The date that the student borrowed the loaner.
    date: String,
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
        .projection_expression("subject, first_name, last_name, date_taken")
        .send()
        .await {
        Ok(output) => {
            if let Some(items) = output.items {
                Ok(items.into_iter().map(|item| {
                    let expandable: ExpandableInfo = from_item(item).unwrap();
                    // This set of function calls is ridiculous, we need to make a better way to do this.
                    LoanerReturnOutput {
                        first_name: expandable.data.get("first_name").unwrap().as_string().unwrap_or_default().unwrap_or_default(),
                        last_name: expandable.data.get("last_name").unwrap().as_string().unwrap_or_default().unwrap_or_default(),
                        date: expandable.data.get("date_taken").unwrap().as_string().unwrap_or_default().unwrap_or_default(),
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
async fn check_in_loaner(student: LoanerReturnOutput) -> Result<(), ServerFnError> {
    let dbclient = Client::new(&aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await);

    match dbclient
        .delete_item()
        .table_name("loaner-info")
        .key("subject", AttributeValue::S(student.subject))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            log!("{:?}", err);
            let msg = err.message().unwrap_or("Unknown error");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server]
async fn create_borrow_entry(input: ExpandableInfo) -> Result<(), ServerFnError> {
    let mut input = input;
    let dbclient = Client::new(&aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await);

    let current_time = chrono::Utc::now().format("%H:%M, %m/%d/%Y").to_string();
    log!("Creating entry with timestamp {}", current_time);

    // Add the timestamp to the data map as a string and then push it to the database.
    input.data.insert("date_taken".to_string(), ValueType::String(Some(current_time.clone())));

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
                        // Retry database writes until we have a good subject ID.
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
/// This page is going to temporarily piggyback on the general scholarship app, at least until
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

    let temp = ExpandableInfo::new(uuid::Uuid::new_v4().to_string());
    let temp_reactive = temp.as_reactive();
    let elements_disabled = RwSignal::new(false);
    
    view! {
        <div class="flex flex-col justify-center gap-2 mb-1 py-3 px-2">
            <h2 class="text-2xl font-bold">"Borrower Information"</h2>
            <p class="text-lg">"Please fill out the form below."</p>
        </div>
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
    let navigate = use_navigate();
    
    // Register server actions
    let check_in_action = ServerAction::<CheckInLoaner>::new();

    // Register server resources
    let return_list_refresh = RwSignal::new(0);
    let return_list_resource: Resource<Vec<LoanerReturnOutput>> = Resource::new(
        move || return_list_refresh.get(),
        async |_| get_loaners_return_list().await.unwrap_or_else(|e| {
            log!("There was an error getting the loaner list: {}", e);
            vec![]
        })
    );

    let dialog_ref = NodeRef::<Dialog>::new();
    let chosen_student = RwSignal::new(None::<LoanerReturnOutput>);
    let dialog_disabled = Signal::derive(move || check_in_action.pending().get());

    // Create an effect to run after check-in - refreshes the list with another scan request.
    // This could be beneficial if we ever decided to make multiple loaner kiosks (think SAT days)
    Effect::new(move || {
        if let Some(Ok(_)) = check_in_action.value().get() {
            // Return to the main page.
            if let Some(dialog) = dialog_ref.get() {
                dialog.close();
            }
            navigate("/loaners", Default::default());
            check_in_action.clear();
        }
    });

    // Handles checking in a student.
    let dialog_check_in = move |_| {
        chosen_student.with(|opt_student| {
            if let Some(student) = opt_student.as_ref() {
                check_in_action.dispatch(CheckInLoaner { student: student.clone() });
            }
        })
    };

    // Handles closing the dialog menu.
    let close_dialog = move |_| {
        if let Some(dialog) = dialog_ref.get() {
            dialog.close();
        }
    };

    view! {
        <dialog
            class="p-3 m-auto rounded-md shadow-lg/33 backdrop:backdrop-blur-xs backdrop:transition-backdrop-filter"
            node_ref=dialog_ref>
            <h2 class="text-2xl font-bold">"Confirmation"</h2>
            <h4 class="text-xl mb-3">
                {move || chosen_student.with(|opt_student| {
                    if let Some(student) = opt_student.as_ref() {
                        format!("Are you sure want to check in {} {}?", student.first_name, student.last_name)
                    } else {
                        "There is no student selected.".to_string()
                    }
                })}
            </h4>
            <ActionButton on:click=close_dialog disabled=dialog_disabled>"Close"</ActionButton>
            <ActionButton on:click=dialog_check_in disabled=dialog_disabled>"Check In"</ActionButton>
        </dialog>
        <div class="flex flex-col justify-center gap-2 mb-3 py-3 px-2">
            <h2 class="text-2xl font-bold">"Student List"</h2>
            <p class="text-lg">"Please select your name from the list."</p>
        </div>
        <Suspense fallback=move || "Loading...".into_view()>
            <div class="flex flex-col gap-2 m-2">
                {move || {
                    match return_list_resource.get() {
                        Some(loaner_list) => {
                            let list_view = loaner_list.iter().map(|student| {
                                let on_click = {
                                    let student_click = student.clone();
                                    move |_| {
                                        log!("Running on_click using student {:?}", student_click);
                                        if let Some(dialog) = dialog_ref.get() {
                                            chosen_student.set(Some(student_click.clone()));
                                            dialog.show_modal().expect("Couldn't show modal dialog.");
                                        }
                                    }
                                };

                                let naive_time = chrono::NaiveDateTime::parse_from_str(&student.date, "%H:%M, %m/%d/%Y")
                                    .unwrap();
                                let time_zone = FixedOffset::west_opt(5 * 3600).unwrap();
                                let student_date = time_zone.from_utc_datetime(&naive_time)
                                    .format("%l:%M %p, %m/%d/%Y")
                                    .to_string();

                                log!("Found current date {:?}", student_date);

                                view! {
                                    <div class="flex flex-row p-3 rounded-md cursor-pointer transition-shadow shadow-sm hover:shadow-lg/33" on:click=on_click>
                                        <div class="flex-1">
                                            {student.first_name.clone()}" "{student.last_name.clone()}
                                        </div>
                                        <div class="flex-none">
                                            {student_date.clone()}
                                        </div>
                                    </div>
                                }
                            }).collect_view();

                            Either::Left(list_view)
                        },
                        None => Either::Right("No loaners found.".into_view())
                    }
                }}
            </div>
        </Suspense>
    }
}
