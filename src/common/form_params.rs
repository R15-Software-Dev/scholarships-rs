use leptos::Params;
use leptos_router::params::Params;

#[derive(Params, PartialEq, Debug)]
pub struct ScholarshipFormParams {
    pub id: Option<String>,
}
