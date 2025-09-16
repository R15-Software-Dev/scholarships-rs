use crate::app::Unauthenticated;
use crate::common::UserClaims;
use crate::components::{Loading, MultiEntry, MultiEntryData, MultiEntryMember, OutlinedTextField};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_oidc::{Algorithm, AuthLoaded, AuthSignal, Authenticated, LogoutLink};

#[component]
pub fn ProviderEntry() -> impl IntoView {
    let auth = use_context::<AuthSignal>().expect("Couldn't find auth signal");
    let auth_data = auth
        .get()
        .authenticated()
        .and_then(|data| data.decoded_access_token::<UserClaims>(Algorithm::RS256, &["account"]));

    let multi_entries: RwSignal<Vec<MultiEntryData>> = RwSignal::new(vec![]);

    console_log(format!("User claims: {auth_data:?}").as_str());

    view! (
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=Unauthenticated>
                <LogoutLink class="text-logout">"Sign out"</LogoutLink>
                <h1>This is the provider entry page</h1>
                <h2>It would usually have a series of questions that we ask the providers</h2>
                <div>
                    <OutlinedTextField
                        placeholder = "Testing information..."
                        name = "testing_input"
                        label = "This is a testing question."
                        value = RwSignal::new("".to_owned())
                    />
                </div>
                <div>
                    <p>This is an example of the multientry element.</p>
                    <MultiEntry
                        entries=multi_entries
                        name_member=Signal::from(MultiEntryMember::from_str("Entry Name", "name"))
                        info_member=Signal::from(MultiEntryMember::from_str("Entry Info", "info"))
                        schema=vec![MultiEntryMember::from_str("Entry Name", "name"), MultiEntryMember::from_str("Entry Info", "info")]
                    />
                </div>
            </Authenticated>
        </AuthLoaded>
    )
}
