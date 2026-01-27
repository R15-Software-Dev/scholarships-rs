use std::collections::HashMap;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use crate::components::{Banner, Loading, OutlinedTextField, Panel, Row, TextFieldType, Toast, ToastContext, ValidatedForm};
use crate::pages::UnauthenticatedPage;
use crate::utils::get_user_claims;
use super::api::{get_provider_contact, PutProviderContact};

#[component]
pub fn ProviderContactPage() -> impl IntoView {
    // Get user claims
    let user_claims = get_user_claims();
    let user_id = Memo::new(move |_| {
        user_claims.get()
            .map(|info| info.claims.subject.clone())
    });

    // Get contact information
    let contact_info = RwSignal::new(HashMap::new());
    let contact_resource = Resource::new(
        move || user_id.get(),
        async move |id| {
            let Some(id) = id else {
                return Ok(HashMap::new());
            };

            get_provider_contact(id).await
        }
    );

    Effect::new(move || {
        match contact_resource.get() {
            Some(Ok(map)) => contact_info.set(map),
            Some(Err(e)) => log!("Error getting contact info: {}", e),
            None => log!("Contact info resource is not ready yet."),
        }
    });

    // Set up submit action
    let submit_action = ServerAction::<PutProviderContact>::new();

    let on_submit = move || {
        // Submit the form by reading the contact_info.
        submit_action.dispatch(PutProviderContact {
            id: user_id.get().unwrap_or_default(),
            contact_info: contact_info.get()
        });
    };

    let elements_disabled = Signal::derive(move || {
        submit_action.pending().get()
    });

    let mut toasts_context = expect_context::<ToastContext>();
    Effect::watch(
        move || submit_action.value().get(),
        move |value, _, _| {
            log!("Running submit_action effect");
            let Some(result) = value else {
                return;
            };
            
            let toast = match result {
                Ok(_) => Toast::new()
                    .id(uuid::Uuid::new_v4())
                    .header("Submission Successful")
                    .msg("You can go back or continue editing your responses."),
                Err(err) => Toast::new()
                    .id(uuid::Uuid::new_v4())
                    .header("Submission Failed")
                    .msg(err.to_string())
            };

            untrack(move || {
                submit_action.clear();
                toasts_context.toast(toast);
            });
        },
        false
    );

// Display contact form.
    view! {
        <Banner title="R15 Scholarships" logo="/PHS_Stacked_Acronym.png" path="/providers" />
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                <Suspense fallback=Loading>
                    <div class="flex flex-row">
                        <div class="flex flex-col flex-1" />
                        <Panel>
                            <ValidatedForm
                                on_submit=on_submit
                                title="Contact Form"
                                description="Enter your contact information and click submit."
                            >
                                {move || {
                                    contact_resource
                                        .get()
                                        .map(|_| {
                                            view! {
                                                <Row>
                                                    <OutlinedTextField
                                                        label="First Name:"
                                                        placeholder="John"
                                                        disabled=elements_disabled
                                                        data_member="first_name"
                                                        data_map=contact_info
                                                        required=true
                                                    />
                                                    <OutlinedTextField
                                                        label="Last Name:"
                                                        placeholder="Smith"
                                                        disabled=elements_disabled
                                                        data_member="last_name"
                                                        data_map=contact_info
                                                        required=true
                                                    />
                                                </Row>
                                                <Row>
                                                    <OutlinedTextField
                                                        label="Contact Email:"
                                                        placeholder="student@region15.org"
                                                        disabled=elements_disabled
                                                        data_member="contact_email"
                                                        data_map=contact_info
                                                        input_type=TextFieldType::Email(vec!["*".to_string()])
                                                        required=true
                                                    />
                                                </Row>
                                                <Row>
                                                    <OutlinedTextField
                                                        label="Phone Number:"
                                                        placeholder="123-456-7890"
                                                        disabled=elements_disabled
                                                        data_member="phone_number"
                                                        data_map=contact_info
                                                        required=true
                                                    />
                                                </Row>
                                                <Row>
                                                    <OutlinedTextField
                                                        label="Street Address:"
                                                        placeholder="123 Fake Street"
                                                        disabled=elements_disabled
                                                        data_member="address"
                                                        data_map=contact_info
                                                        required=true
                                                    />
                                                </Row>
                                            }
                                        })
                                }}
                            </ValidatedForm>
                        </Panel>
                        <div class="flex flex-col flex-1" />
                    </div>
                </Suspense>
            </Authenticated>
        </AuthLoaded>
    }
}