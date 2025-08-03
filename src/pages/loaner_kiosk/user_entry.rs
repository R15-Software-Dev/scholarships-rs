//! Contains the user entry component.
//! It would be preferred if the user was restricted to a real user account. This is the URL:
//! https://admin.googleapis.com/admin/directory/v1/users?domain=region15.org&projection=BASIC&query=isArchived=false%20isAdmin=false
//! Once they select this, they can then check in/out the loaners under their account.
//! That means that the Leptos server will now REQUIRE a user token of some sort, probably
//! a service account. That's going to be a pain to set up.
//! For example's sake here, we're just going to take a first name, last name, and email address.

use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use crate::common::LoanerUser;
use crate::components::{ActionButton, OutlinedTextField};

/// The user entry component for the loaner kiosk.
///
/// Users are able to input their first name, last name, and email address. In the future, this
/// component should call out to a server function that allows the application to check if the
/// information provided is correct. This may be by creating storing a series of user records
/// or simply querying Google's Admin API to check if the user is valid.
#[component]
pub fn UserEntry(
    /// A callback function used when the control has determined this stage is finished.
    #[prop()] mut on_submit: impl FnMut() + 'static,
    /// The global user record.
    #[prop()] user: RwSignal<LoanerUser>,
    /// Whether this control is shown. Required for use with CSS animations.
    #[prop()] shown: bool
) -> impl IntoView {
    let disabled = RwSignal::new(false);
    let on_click = move |_| {
        disabled.set(true);
        on_submit();
        console_log("Clicked!");
        disabled.set(false);
    };

    view! {
        <div
            class="transition-all duration-300 flex flex-col"
            class=(["opacity-100 delay-300"], shown)
            class=(["opacity-0"], !shown)
        >
            <OutlinedTextField
                label="First Name".into()
                placeholder="John".into()
                disabled={disabled}
            />
            <OutlinedTextField
                label="Last Name".into()
                placeholder="Smith".into()
                disabled={disabled}
            />
            <OutlinedTextField
                label="Region 15 Email".into()
                placeholder="jsmith@region15.org".into()
                disabled={disabled}
            />
            <ActionButton
                on:click=on_click
                disabled={disabled}
            >
                "Submit"
            </ActionButton>
        </div>
    }
}
