//! Login page of the application.

use crate::router::AppRoute;
use crate::stores::UserState;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::components::LoginForm;

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let (user, _dispatch) = use_store::<UserState>();

    if user.is_logged_in() {
        navigator.push(&AppRoute::Comments);
    }

    html! {
        <div class="fullscreen_center_app">
            <div class="login_box">
                <h2>{"Login"}</h2>
                <LoginForm />
            </div>
        </div>
    }
}
