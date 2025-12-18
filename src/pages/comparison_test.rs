use leptos::either::Either;
use super::home_page::get_submission;
use super::{UnauthenticatedPage};
use crate::common::{
    ComparisonData, ExpandableInfo, ValueType,
};
use crate::components::{ActionButton, Loading, Panel, Row};
use leptos::leptos_dom::logging::console_log;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use crate::pages::utils::get_user_claims;
use super::api::{get_comparison_info, get_all_scholarship_info};

#[component]
pub fn ComparisonTestPage() -> impl IntoView {
    let result_msg = RwSignal::new("Found submission, waiting for input.".to_string());
    let user_claims = get_user_claims();

    // Note that the value passed in MUST be equatable.
    // We get/unwrap the value repeatedly until we get a simple string value, then clone it so that
    // we don't lose access to it in the future, should we need it again.
    let student_resource: Resource<ExpandableInfo> = Resource::new(
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
    
    let scholarship_resource: Resource<Vec<ExpandableInfo>> = Resource::new(
        move || Option::<String>::None,
        async |_| {
            get_all_scholarship_info().await.unwrap_or_else(|e| {
                log!("An error occurred: {:?}", e);
                Vec::new()
            })
        }
    );
    
    let comparison_resource: Resource<Vec<ComparisonData>> = Resource::new(
        move || Option::<String>::None,
        async |_| {
            get_comparison_info().await.unwrap_or_else(|e| {
                log!("An error occurred: {:?}", e);
                Vec::new()
            })
        }
    );

    view! {
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                <Suspense fallback=Loading>
                    {move || {
                        let student_info = student_resource.get();
                        let comparison_info = comparison_resource.get();
                        let scholarship_info = scholarship_resource.get();

                        match (student_info, comparison_info, scholarship_info) {
                            (Some(student), Some(comparisons), Some(scholarships)) => Either::Right(view! {
                                <Panel>
                                    <Row>
                                        <p>{result_msg}</p>
                                    </Row>
                                    <Row>
                                        <ActionButton on:click=move |_| {
                                            // Iterate over all scholarships, get the comparison IDs,
                                            // get ComparisonData from the IDs, then perform the
                                            // comparisons and return a boolean. Alternatively, return
                                            // a list of comparisons that complete and a list that don't.
                                            let comp_member = "requirements".to_string();
                                            let comparison_results = scholarships.iter().map(|scholarship| {
                                                let list = match scholarship.data.get(&comp_member) {
                                                    Some(ValueType::List(Some(list))) => list,
                                                    _ => {
                                                        log!("Cannot find requirements, assuming none.");
                                                        return Ok(Some(scholarship))  // students will always qualify when there are no requirements
                                                    }
                                                };
                                        
                                                let mut final_result = Ok(true);
                                    
                                                // The list should exist, and it should contain a series of strings.
                                                for val in list {
                                                    let comp_opt = match val {
                                                        ValueType::String(Some(comp_id)) => {
                                                            // Find the comparison
                                                            comparisons.iter().find(|comp| comp.id == *comp_id)
                                                        }
                                                        _ => None
                                                    };
                                        
                                                    let comparison_outcome = match comp_opt {
                                                        Some(comp) => {
                                                            // Perform the comparison
                                                            comp.compare(&student)
                                                        }
                                                        _ => Err("".to_string())
                                                    };
                                            
                                                    match comparison_outcome {
                                                        Ok(true) => {},
                                                        Ok(false) => final_result = Ok(false),
                                                        Err(e) => {
                                                            final_result = Err(e);
                                                            break;
                                                        }
                                                    }
                                                }
                                    
                                                match final_result {
                                                    Ok(true) => Ok(Some(scholarship)),
                                                    Ok(false) => Ok(None),
                                                    Err(e) => Err(e)
                                                }
                                            }).collect::<Vec<Result<Option<&ExpandableInfo>, String>>>();
                                            
                                            // Build a display string
                                            let mut strings = Vec::new();
                                            strings.push("Scholarship results: \n".to_string());
                                            for result in comparison_results {
                                                match result {
                                                    Ok(Some(scholarship_info)) => {
                                                        // Get the scholarship name.
                                                        let name_member = "name".to_string();
                                                        let scl_name = scholarship_info.data.get(&name_member).unwrap()
                                                            .as_string().unwrap().unwrap();
                                                        strings.push(scl_name);
                                                    },
                                                    Ok(None) => {},
                                                    Err(msg) => strings.push(msg)
                                                }
                                            }
                                
                                            let display_text = strings.join("\n").to_string();
                                
                                            result_msg.set(display_text);
                                        }>"Test Comparison"</ActionButton>
                                    </Row>
                                </Panel>
                            }),
                            _ => Either::Left(view! {
                                <p>"Failed API requests."</p>
                            })
                        }
                    }}
                </Suspense>
            </Authenticated>
        </AuthLoaded>
    }
}
