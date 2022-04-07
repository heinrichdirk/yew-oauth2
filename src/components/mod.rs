mod auth;
mod failure;
mod noauth;
mod oauth2;
mod redirect;
#[cfg(feature = "yew-router-nested")]
mod router;

pub use self::oauth2::*;
pub use auth::*;
pub use failure::*;
pub use noauth::*;
pub use redirect::*;
#[cfg(feature = "yew-router-nested")]
pub use router::*;

use crate::prelude::*;
use yew::prelude::*;

fn missing_context() -> Html {
    html!(<div> { "Unable to find OAuth2 context! You need to wrap this element into a `OAuth2` component"} </div>)
}

fn start_login() {
    log::info!("Triggering login");
    OAuth2Dispatcher::new().start_login();
}
