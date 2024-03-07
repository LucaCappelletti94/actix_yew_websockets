//! Login page of the application.

use crate::router::AppRoute;
use crate::stores::UserState;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

#[function_component(Comments)]
pub fn comments() -> Html {
    let navigator = use_navigator().unwrap();
    let (user, _) = use_store::<UserState>();

    if user.is_logged_in() {
        navigator.push(&AppRoute::Login);
    }

    html! {
        <div class="fullscreen_center_app">
            <div class="login_box">
                <h2>{"Login"}</h2>
                
            </div>
        </div>
    }
}
