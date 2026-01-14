use std::collections::HashMap;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use crate::common::ValueType;
use crate::components::{Banner, Loading, OutlinedTextField, Panel, Row, TextFieldType, Toast, ToastContext, ToastList, ValidatedForm};
use crate::pages::UnauthenticatedPage;
use crate::pages::utils::get_user_claims;

#[cfg(feature = "ssr")]
use crate::pages::utils::server_utils::create_dynamo_client;

#[cfg(feature = "ssr")]
static PROVIDER_CONTACT_TABLE: &str = "leptos-provider-contacts";

#[server]
async fn get_provider_contact(id: String) -> Result<HashMap<String, ValueType>, ServerFnError> {
    use aws_sdk_dynamodb::types::AttributeValue;
    use aws_sdk_dynamodb::error::ProvideErrorMetadata;

    let client = create_dynamo_client().await;

    log!("Getting contact info for provider {}", id);

    match client
        .get_item()
        .table_name(PROVIDER_CONTACT_TABLE)
        .key("subject", AttributeValue::S(id))
        .send()
        .await
    {
        Ok(output) => {
            let Some(item) = output.item else {
                return Ok(HashMap::new());
            };

            let map = item.iter().map(|(key, val)| {
                let val_type = ValueType::from(val);

                (key.clone(), val_type)
            }).collect::<HashMap<String, ValueType>>();

            Ok(map)
        }
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred.");
            Err(ServerFnError::new(msg))
        }
    }
}

#[server]
async fn put_provider_contact(id: String, contact_info: HashMap<String, ValueType>) -> Result<(), ServerFnError> {
    use aws_sdk_dynamodb::error::ProvideErrorMetadata;
    use crate::pages::utils::server_utils::into_attr_map;

    let client = create_dynamo_client().await;

    let mut item = into_attr_map(contact_info);
    item.insert("subject".to_owned(), aws_sdk_dynamodb::types::AttributeValue::S(id));

    match client
        .put_item()
        .table_name(PROVIDER_CONTACT_TABLE)
        .set_item(Some(item))
        .send()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            let msg = err.message().unwrap_or("An unknown error occurred.");
            Err(ServerFnError::new(msg))
        }
    }
}

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
    Effect::new(move || {
        submit_action.value().get().is_some()
            .then(|| {
                toasts_context.toast(
                    Toast::new()
                        .id(uuid::Uuid::new_v4())
                        .header("Submission Successful")
                        .msg("You can go back or continue editing your responses.")
                );
                
                submit_action.clear();
            });
    });
    
    // Display contact form.
    view! {
        <Banner title="R15 Scholarships" logo="/PHS_Stacked_Acronym.png" path="/" />
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
                                                        input_type=TextFieldType::Email
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
                                                <Row>
                                                    <OutlinedTextField
                                                        label="Testing Number"
                                                        placeholder="Just put a number"
                                                        disabled=elements_disabled
                                                        data_member="testing_number"
                                                        data_map=contact_info
                                                        input_type=TextFieldType::Number
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