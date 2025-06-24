//! This defines the base component of the Chromebook loaner kiosk application.
//! There are going to be 3 definitive screens that the users are able to see and use:
//!  - The username entry screen: users will enter their name and email.
//!  - The action selection screen: users will be given a choice regarding their loaner status.
//!  - The completion screen: users will be given a success message should their information save.
//! Refer to each component's file for more information about how they work.

use leptos::prelude::*;
use crate::common::LoanerUser;
use super::{
    user_entry::UserEntry,
    action_selection::ActionSelection
};

/// Used to keep track of the currently displayed controls.
#[derive(Clone, Copy, PartialEq)]
pub enum LoanerKioskState {
    UserEntry,
    ActionSelection,
    Complete
}

/// The base Chromebook loaner kiosk component.
///
/// Controls the state of the loaner application and gives a space for the subcomponents to display.
#[component]
pub fn LoanerKiosk() -> impl IntoView {
    let current_state = RwSignal::new(LoanerKioskState::UserEntry);
    let user = RwSignal::new(LoanerUser::new());
    
    view! {
        <div class="flex w-screen h-screen justify-center items-center">
            <UserEntry
                on_submit={move || { current_state.set(LoanerKioskState::ActionSelection); }}
                user={user}
                shown={current_state.get() == LoanerKioskState::UserEntry}
            />
            <ActionSelection
            />
        </div>
    }
}
