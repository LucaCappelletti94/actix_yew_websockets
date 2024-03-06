use crate::components::counter::*;
use crate::worker::*;
use yew::prelude::*;
use yew_agent::worker::WorkerProvider;

#[function_component]
#[allow(non_snake_case)]
pub fn App() -> Html {
    html! {
        <div class="app">
            <WorkerProvider<WebsocketWorker> path="web_socket_worker.js">
                <Counter />
            </WorkerProvider<WebsocketWorker>>
        </div>
    }
}
