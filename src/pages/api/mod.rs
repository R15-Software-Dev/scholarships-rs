mod admin;
mod comparisons;
mod dates;
pub mod exports;
mod providers;
mod scholarships;
pub mod students;

pub use admin::*;
pub use comparisons::*;
pub use dates::*;
pub use providers::*;
pub use scholarships::*;

#[cfg(feature = "ssr")]
pub static COMPARISONS_TABLE: &str = "leptos-comparisons";

#[cfg(feature = "ssr")]
pub static DATES_TABLE: &str = "leptos-dates";

#[cfg(feature = "ssr")]
pub static SCHOLARSHIPS_TABLE: &str = "leptos-scholarships";

#[cfg(feature = "ssr")]
pub static PROVIDER_CONTACT_TABLE: &str = "leptos-provider-contacts";

#[cfg(feature = "ssr")]
pub static MAIN_TABLE_NAME: &str = "scholarships-main";
