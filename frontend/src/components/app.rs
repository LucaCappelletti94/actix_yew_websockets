use crate::router::{switch, AppRoute};
use crate::worker::*;
use commons::messages::{BackendMessage, FrontendMessage};
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;
use yew_router::prelude::*;

#[function_component]
#[allow(non_snake_case)]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="fullscreen_center_app">
                <WorkerProvider<WebsocketWorker<FrontendMessage, BackendMessage>> path="web_socket_worker.js">
                <Switch<AppRoute> render={switch} />
            </WorkerProvider<WebsocketWorker<FrontendMessage, BackendMessage>>>
            </div>
        </BrowserRouter>
    }
}
