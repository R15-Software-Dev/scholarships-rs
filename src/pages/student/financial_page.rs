use crate::components::{FileDrop, Header, Loading};
use crate::pages::api::files::list_files;
use leptos::prelude::*;
use leptos_oidc::AuthSignal;

#[component]
pub fn StudentFinancialPage() -> impl IntoView {
    // This is where we'll get information about the files that have been uploaded.
    // For now, we'll leave it blank.

    let auth = expect_context::<AuthSignal>();

    let files_resource = Resource::new(
        move || {
            auth.try_with(|a| a.authenticated().map(|a| a.access_token()))
                .flatten()
        },
        async move |access_token| {
            list_files(
                access_token.unwrap_or_default(),
                "financial_info".to_string(),
                "fafsa".to_string(),
            )
            .await
        },
    );

    view! {
        <div class="flex flex-1" />
        <div class="flex flex-col flex-2 mt-6">
            // <ValidatedForm title="Student Demographic Form" on_submit=controller.submit_action>
            <Header
                title="Financial Information"
                description="Please upload your FAFSA SAR (Student Aid Report) here."
            />
            <Suspense fallback=Loading>
                <FileDrop
                    name="fafsa"
                    form_id="financial_info"
                    existing_files=files_resource
                    allowed_types=vec![".pdf".to_string()]
                />
            </Suspense>
        // </ValidatedForm>
        </div>
        <div class="flex flex-1" />
    }
}
