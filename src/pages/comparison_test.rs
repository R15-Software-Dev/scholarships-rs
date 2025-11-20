use super::home_page::get_submission;
use super::UnauthenticatedPage;
use crate::common::{
    ComparisonData, ComparisonType, ExpandableInfo, NumberComparison, UserClaims, ValueType,
};
use crate::components::{ActionButton, Loading, Panel, Row};
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_oidc::{Algorithm, AuthLoaded, AuthSignal, Authenticated};

#[component]
pub fn ComparisonTestPage() -> impl IntoView {
    let result_msg = RwSignal::new("Found submission, waiting for input.".to_string());
    let auth = use_context::<AuthSignal>().expect("Couldn't find user information.");
    let user_claims = Signal::derive(move || {
        auth.with(|auth| {
            auth.authenticated().and_then(|data| {
                data.decoded_access_token::<UserClaims>(Algorithm::RS256, &["account"])
            })
        })
    });

    // Note that the value passed in MUST be equatable.
    // We get/unwrap the value repeatedly until we get a simple string value, then clone it so that
    // we don't lose access to it in the future, should we need it again.
    let server_resource: Resource<ExpandableInfo> = Resource::new(
        move || user_claims.get().map(|claim| claim.claims.subject.clone()),
        async
            | opt_username
            | match opt_username {
                Some(subject) => get_submission(subject.clone()).await.unwrap_or_else(|e| {
                    console_log(e.to_string().as_str());
                    ExpandableInfo::new(subject)
                }),
                None => ExpandableInfo::new("".to_owned()),
            },
    );

    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                <Suspense fallback=Loading>
                    {move || {
                        server_resource
                            .get()
                            .map(|response| {
                                view! {
                                    <Panel>
                                        <Row>
                                            <p>{result_msg}</p>
                                        </Row>
                                        <Row>
                                            <ActionButton on:click=move |_| {
                                                let comp = ComparisonData {
                                                    id: "".into(),
                                                    member: "math_sat".into(),
                                                    target_value: ValueType::Number(Some("800".into())),
                                                    comparison: ComparisonType::Number(
                                                        NumberComparison::GreaterThan,
                                                    ),
                                                    category: "".into(),
                                                    display_text: "".into(),
                                                };
                                                let result = comp.compare(&response);
                                                match result {
                                                    Ok(val) => {
                                                        result_msg
                                                            .set(
                                                                format!(
                                                                    "member {} is greater than target value {}: {}",
                                                                    comp.member,
                                                                    comp.target_value,
                                                                    val,
                                                                ),
                                                            )
                                                    }
                                                    Err(msg) => result_msg.set(msg),
                                                }
                                            }>"Test Comparison"</ActionButton>
                                        </Row>
                                    </Panel>
                                }
                            })
                    }}
                </Suspense>
            </Authenticated>
        </AuthLoaded>
    }
}
