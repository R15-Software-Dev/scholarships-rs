use crate::components::{FileDrop, Header};
use leptos::prelude::*;

#[component]
pub fn StudentFinancialPage() -> impl IntoView {
    // This is where we'll get information about the files that have been uploaded.
    // For now, we'll leave it blank.

    view! {
        <div class="flex flex-1" />
        <div class="flex flex-col flex-2 mt-6">
            // <ValidatedForm title="Student Demographic Form" on_submit=controller.submit_action>
            <Header
                title="Financial Information"
                description="Please upload your FAFSA SAR (Student Aid Report) here."
            />
            <FileDrop name="fafsa" form_id="financial_info" />
        // </ValidatedForm>
        </div>
        <div class="flex flex-1" />
    }
}
