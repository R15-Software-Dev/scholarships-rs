use chrono::{DateTime, Utc};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use leptos::logging::log;

#[cfg(feature = "ssr")]
use aws_sdk_dynamodb::{Client, error::ProvideErrorMetadata, types::AttributeValue};

#[cfg(feature = "ssr")]
use serde_dynamo::{from_item, to_item};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoanerReturnOutput {
    /// The first name of the student that is borrowing.
    name: String,
    /// The last name of the student that is borrowing.
    last_name: String,
    /// The type of loan that was given to the student.
    loan: String,
    /// The date that the student borrowed the loaner.
    date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LoanerBorrowInput {
    /// The first name of the student that is borrowing.
    first_name: String,
    /// The last name of the student that is borrowing.
    last_name: String,
    /// The email address of the student that is borrowing.
    email: String,
    /// The collateral that the student is leaving. Usually a phone, earbuds, etc.
    collateral: String,
    /// The type of loan that was given to the student. May be a Chromebook, headphones, etc.
    loan: String,
    /// The date that the loaner was borrowed.
    date: DateTime<Utc>,
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
        .projection_expression("name, date")
        .send()
        .await {
        Ok(output) => {
            if let Some(items) = output.items {
                Ok(items.into_iter().map(|item| from_item(item).unwrap()).collect())
            } else {
                Ok(vec![])
            }
        },
        Err(err) => {
            let msg = err.message().unwrap_or("Unknown error");
            Err(ServerFnError::new(msg))
        },
    }
}

#[server]
async fn create_borrow_entry(input: LoanerBorrowInput) -> Result<(), ServerFnError> {
    let dbclient = Client::new(&aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await);
    let item = to_item(input)?;

    match dbclient
        .put_item()
        .table_name("loaner-info")
        .set_item(Some(item))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred.");
            Err(ServerFnError::new(msg))
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
    // Register server actions.
    let create_entry_action = ServerAction::<CreateBorrowEntry>::new();

    // Register server resources
    let return_list_resource: Resource<Vec<LoanerReturnOutput>> = Resource::new(
        move || (),
        async |_| get_loaners_return_list().await.unwrap_or_else(|e| {
            log!("There was an error getting the loaner list: {}", e);
            vec![]
        })
    );

    // We'll create the view here. Just remember that we want to have two sections, where they
    // preferably slide in when they're activated.
    view! {}
}
