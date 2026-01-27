mod comparisons;
mod scholarships;
mod dates;
mod admin;
mod providers;

pub use comparisons::*;
pub use scholarships::*;
pub use dates::*;
pub use admin::*;
pub use providers::*;

#[cfg(feature = "ssr")]
pub static COMPARISONS_TABLE: &str = "leptos-comparisons";

#[cfg(feature = "ssr")]
pub static DATES_TABLE: &str = "leptos-dates";

#[cfg(feature = "ssr")]
pub static SCHOLARSHIPS_TABLE: &str = "leptos-scholarships";

#[cfg(feature = "ssr")]
pub static PROVIDER_CONTACT_TABLE: &str = "leptos-provider-contacts";
