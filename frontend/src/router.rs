//! Router of the single-page application.

use crate::pages::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum AppRoute {
    #[at("/")]
    Login,
    #[at("/comments")]
    Comments,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(switch: AppRoute) -> Html {
    match switch {
        AppRoute::Login => html! {<Login />},
        AppRoute::Comments => html! {<Comments />},
        AppRoute::NotFound => html! {<NotFound />},
    }
}
