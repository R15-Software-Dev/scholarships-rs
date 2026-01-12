use std::time::Duration;
use leptos::prelude::*;
use leptos_animate::animate;
use leptos_animate::animations::classes::{In, Out};

#[component]
pub fn Toast(
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] message: Signal<String>,
    #[prop(into)] title: Signal<String>
) -> impl IntoView {
    set_timeout(
        move || {
            on_close.run(())
        },
        Duration::from_secs(3)
    );
    
    view! {
        <div
            class="flex flex-col rounded-md shadow-lg/33 bg-white p-3 m-1.5 mx-auto min-w-lg max-w-lg"
            use:animate=(
                In::default()
                    .source("opacity-0 -translate-y-2 scale-90")
                    .active("transition-all duration-150 transform-gpu")
                    .target("opacity-100 translate-y-0 scale-100"),
                Out::default()
                    .source("opacity-100 translate-y-0 scale-100")
                    .active("transition-all duration-150 transform-gpu")
                    .target("opacity-0 -translate-y-2 scale-90"),
            )
        >
            <h3 class="font-bold text-lg">{title}</h3>
            <p>{message}</p>
        </div>
    }
}
