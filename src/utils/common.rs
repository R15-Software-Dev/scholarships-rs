use leptos::prelude::*;
use leptos_oidc::{Algorithm, AuthSignal, TokenData};
use crate::common::UserClaims;

pub fn use_origin() -> String {
    #[cfg(target_arch = "wasm32")] {
        use leptos::web_sys;
        // Just get the current URL origin.
        web_sys::window()
            .unwrap()
            .location()
            .origin()
            .unwrap_or("http://localhost:3000/".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))] {
        // Read the expected origin out of an environment variable.
        std::env::var("LP_SITE_ORIGIN").unwrap_or("http://localhost:3000/".to_string())
    }
}

/// Gets the current user claims. This function should only be used in an area that has
/// access to an AuthSignal, or it will result in a total failure.
pub fn get_user_claims() -> Signal<Option<TokenData<UserClaims>>> {
    let auth = use_context::<AuthSignal>().expect("Couldn't find AuthSignal.");
    Signal::derive(move || {
        auth.with(|auth| {
            auth.authenticated().and_then(|data| {
                data.decoded_access_token::<UserClaims>(Algorithm::RS256, &["account"])
            })
        })
    })
}
