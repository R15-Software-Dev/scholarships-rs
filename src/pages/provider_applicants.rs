use crate::common::ValueType;
use crate::components::{Banner, Loading};
use leptos::either::Either;
use leptos::prelude::*;
use leptos_oidc::AuthSignal;

#[component]
pub fn ApplicantsPageFallback() -> impl IntoView {
    view! {
        <Banner title="R15 Scholarships" logo="/PHS_Stacked_Acronym.png" path="/providers" />
        <div class="flex flex-col gap-4 mt-3 items-center justify-center">
            <h1 class="text-3xl font-bold">"Applicants Page"</h1>

            <p>
                "This page is under construction! It will show all the students that are eligible for your scholarship(s)."
            </p>
            <p>"Come back soon!"</p>
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

    view! { <div></div> }
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

    view! {
        <div class="flex flex-col rounded-md shadow-lg/33 p-2">
            <h2 class="text-xl font-bold flex-1 text-center">"Scholarships"</h2>
            <Transition fallback=Loading>
                {move || {
                    scholarships_res
                        .get()
                        .map(|items_res| {
                            let items = match items_res {
                                Ok(items) => items,
                                Err(e) => {
                                    return Either::Left(
                                        view! {
                                            <div>
                                                {format!("Couldn't get scholarships: {}", e.to_string())}
                                            </div>
                                        }
                                            .into_any(),
                                    );
                                }
                            };
                            Either::Right(
                                items
                                    .into_iter()
                                    .map(|item| {

                                        view! { <ApplicantsScholarshipEntry /> }
                                            .into_any()
                                    })
                                    .collect_view(),
                            )
                        })
                        .collect_view()
                }}
            </Transition>
        </div>
    }
}

#[component]
fn ApplicantsScholarshipEntry() -> impl IntoView {}

#[component]
fn ApplicantsStudentList() -> impl IntoView {}
