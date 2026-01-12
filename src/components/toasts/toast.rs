use std::time::Duration;
use leptos::prelude::*;
use leptos_animate::animate;
use leptos_animate::animations::classes::In;

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
        <div class="flex flex-col rounded-md shadow-md"
            use:animate=In::default()
                .source("opacity-0 -translate-y-2 -scale-10")
                .active("transition-all duration-150")
                .target("opacity-100")>
            <h3 class="font-bold text-lg">{title}</h3>
            <p>{message}</p>
        </div>
    }
}
