use crate::common::{ExpandableInfo, ValueType};
use crate::components::{Banner, Loading};
use leptos::prelude::*;
use leptos_oidc::AuthSignal;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_navigate;

#[component]
pub fn ApplicantsPageFallback() -> impl IntoView {
    view! {
        <div class="flex flex-col flex-1 gap-2">
            <h1 class="flex-1 text-center font-bold text-2xl">"Scholarship Applicants"</h1>
            <p class="text-center text-lg">
                "Please select a scholarship from the left to view the eligible applicants."
            </p>
        </div>
    }
}

#[component]
pub fn ApplicantsPageShell() -> impl IntoView {
    // This shell needs to simply contain everything that will stay consistent across paths.
    // The path will change to contain each scholarship's id, which will directly affect the content
    // shown in a subroute for this page.

    // As a result, this page likely only needs to contain the banner and the list of scholarships.
    // The content will be shown within the right side, but only as a result of some subroutes,
    // each of which is still considered a single page.

    view! {
        <Banner title="R15 Scholarships" logo="/PHS_Stacked_Acronym.png" path="/providers" />
        <div class="flex flex-row mt-5">
            <div class="flex-1" />
            <div class="flex-3 flex flex-row">
                <ApplicantsScholarshipList />
                <div class="flex-3 text-center">
                    <Outlet />
                </div>
            </div>
            <div class="flex-1" />
        </div>
    }
}

#[component]
fn ApplicantsScholarshipList() -> impl IntoView {
    use crate::common::ExpandableInfo;

    #[server]
    async fn get_provider_scholarships(
        access_token: String,
    ) -> Result<Vec<ExpandableInfo>, ServerFnError> {
        use crate::pages::api::SCHOLARSHIPS_TABLE;
        use crate::pages::api::tokens::validate_and_get_token_info;

        let claims = validate_and_get_token_info(access_token).await?;

        // Get the information from the database.
        let client = crate::utils::server::create_dynamo_client().await;

        let output = client
            .scan()
            .table_name(SCHOLARSHIPS_TABLE)
            .expression_attribute_values(
                ":id",
                serde_dynamo::to_attribute_value(ValueType::String(Some(claims.subject)))?,
            )
            .filter_expression("provider_id = :id")
            .send()
            .await?
            .items
            .unwrap_or_default()
            .into_iter()
            .filter_map(|item| serde_dynamo::from_item(item).ok())
            .collect::<Vec<ExpandableInfo>>();

        Ok(output)
    }

    // Use access token for user identification
    let auth = expect_context::<AuthSignal>();
    let access_token =
        Memo::new(move |_| auth.with(|auth| auth.authenticated().map(|a| a.access_token())));

    // We need to get the scholarships from the API. We don't need the whole scholarship, just the
    // name and ID.
    let trigger = Trigger::new();
    let scholarships_res = Resource::new(
        move || (trigger.track(), access_token.get()),
        async move |(_, access_token)| {
            get_provider_scholarships(access_token.unwrap_or_default()).await
        },
    );

    let navigate = use_navigate();

    // This closure returns another closure that's using the correct scholarship ID.
    // It avoids having to define it within the Transition component. It's wrapped in a Callback
    // because this makes it Send + Sync.
    let on_view_click = Callback::new(move |scholarship_id: String| {
        navigate(&*scholarship_id, Default::default());
    });

    view! {
        <div class="flex flex-col flex-1 rounded-md shadow-lg/33 p-2 gap-3">
            <h2 class="text-xl font-bold flex-1 text-center">"Scholarships"</h2>
            <Transition fallback=Loading>
                {move || {
                    scholarships_res
                        .get()
                        .map(|items_res| {
                            let items = match items_res {
                                Ok(items) => items,
                                Err(e) => {
                                    return view! {
                                        <div>
                                            {format!("Couldn't get scholarships: {}", e.to_string())}
                                        </div>
                                    }
                                        .into_any();
                                }
                            };

                            view! {
                                <For
                                    each=move || items.clone()
                                    key=|item| item.subject.clone()
                                    let(item)
                                >
                                    <ApplicantsScholarshipEntry
                                        item=item
                                        on_view_click=on_view_click
                                    />
                                </For>
                            }
                                .into_any()
                        })
                        .collect_view()
                }}
            </Transition>
        </div>
    }
}

#[component]
fn ApplicantsScholarshipEntry(
    #[prop()] item: ExpandableInfo,
    #[prop(into)] on_view_click: Callback<String, ()>,
) -> impl IntoView {
    // Store the item's subject.
    let subject = StoredValue::new(item.subject);
    let scholarship_name = Memo::new(move |_| {
        item.data
            .get("name")
            .map(|name| name.as_string().ok().flatten())
            .flatten()
            .unwrap_or("<unnamed scholarship>".to_string())
    });

    let on_click = move |_| {
        on_view_click.run(subject.get_value());
    };

    view! {
        <div class="flex flex-col rounded-md shadow-md hover:shadow-lg/33 transition-shadow">
            <div class="text-center p-2">{scholarship_name}</div>
            <div
                class="flex-1 font-bold text-white rounded-b-md text-center cursor-pointer bg-red-800 hover:bg-red-900 p-2 transition-bg"
                on:click=on_click
            >
                "View Applicants"
            </div>
        </div>
    }
}

#[component]
fn ApplicantsStudentList() -> impl IntoView {}
