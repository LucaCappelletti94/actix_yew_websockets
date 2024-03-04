//! This is the main file for the frontend. It will render the App component.
use yew::prelude::*;
use log::info;
mod reactors;
use crate::reactors::*;
use yew_agent::reactor::ReactorProvider;


#[function_component]
#[allow(non_snake_case)]
pub fn App() -> Html {
    info!("Rendering App component.");

    html! {
        <div class="app">
            <ReactorProvider<WebsocketReactor> path="socket.js" />
        </div>
    }
}

fn main() {
    // We initialize the logger for the frontend
    wasm_logger::init(wasm_logger::Config::default());
    // And then we render the App component
    yew::Renderer::<App>::new().render();
}
