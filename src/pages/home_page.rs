use leptos::prelude::*;

use crate::components::ActionButton;

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let button_disabled = RwSignal::new(false);
    let on_click = move |_| {
        *count.write() += 1;
        *button_disabled.write() = count.get() == 25;
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <ActionButton on:click=on_click disabled={button_disabled}>"Click Me: " {count}</ActionButton>
    }
}
