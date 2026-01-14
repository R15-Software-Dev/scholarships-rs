use leptos_animate::animations::classes::{In, Out};

pub fn pop_in_out() -> (In, Out) {
    (
        In::default()
            .source("opacity-0 -translate-y-2 scale-90")
            .active("transition-all duration-150 transform-gpu")
            .target("opacity-100 translate-y-0 scale-100"),
        Out::default()
            .source("opacity-100 translate-y-0 scale-100")
            .active("transition-all duration-150 transform-gpu")
            .target("opacity-0 -translate-y-2 scale-90"),
    )
}
