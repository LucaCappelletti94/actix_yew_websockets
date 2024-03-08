//! Login page of the application.

use crate::router::AppRoute;
use crate::stores::UserState;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;
use crate::components::comments_dashboard::CommentsDashboard;

#[function_component(Comments)]
pub fn comments() -> Html {
    let navigator = use_navigator().unwrap();
    let (user, _) = use_store::<UserState>();

    if !user.is_logged_in() {
        navigator.push(&AppRoute::Login);
    }

    let user = user.get_user().unwrap();

    html! {
        <CommentsDashboard user={user} />
    }
}
