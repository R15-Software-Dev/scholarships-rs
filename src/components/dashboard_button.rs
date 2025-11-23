use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
#[component]
pub fn DashboardButton(
    #[prop(into)] title: String,
    #[prop(into, optional)] description: Option<String>,
    #[prop(into, optional)] icon: Option<String>,
    #[prop(into)] path: String,
) -> impl IntoView {
    let navigate = use_navigate();

    let on_click = move |_| navigate(&path, Default::default());

    view! {
        <button
            type="button"
            class="dashboard-button flex items-start gap-3 rounded-lg border p-4
                   hover:bg-gray-100 transition cursor-pointer w-full text-left"
            on:click=on_click
        >
            {icon.as_ref().map(|src| view! {
                <img src={src.clone()} class="h-8 w-8" alt="icon"/>
            })}

            <div class="flex flex-col">
                <h3 class="font-semibold text-base">{title.clone()}</h3>
                {description.as_ref().map(|d| view! {
                    <p class="text-sm text-gray-600">{d.clone()}</p>
                })}
            </div>
        </button>
    }
}
