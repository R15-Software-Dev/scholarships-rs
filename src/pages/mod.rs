mod about_page;
mod api;
mod comparison_test;
mod home_page;
mod loaner_page;
mod provider_portal;
mod scholarship_info;
mod unauthenticated_page;
mod provider_contact;
mod auth_callback;
mod provider_applicants;
mod admin;
pub(crate) mod student;

pub use self::{
    about_page::*, comparison_test::*, home_page::*, loaner_page::*, provider_portal::*,
    unauthenticated_page::*, scholarship_info::*, provider_contact::*,
    auth_callback::*, provider_applicants::*, admin::*, student::*,
};
