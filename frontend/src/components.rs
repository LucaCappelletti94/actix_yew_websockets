//! This is the main file for the frontend. It will render the App component.
use crate::worker::*;
use commons::messages::{BackendMessage, FrontendMessage};
use log::info;
use yew::prelude::*;
use yew_agent::prelude::*;
use yew_agent::worker::WorkerProvider;

#[function_component]
#[allow(non_snake_case)]
pub fn App() -> Html {
    info!("Rendering App component.");

    html! {
        <div class="app">
            <WorkerProvider<WebsocketWorker> path="web_socket_worker.js">
                <SubscriberExample />
                <GaussExample />
            </WorkerProvider<WebsocketWorker>>
        </div>
    }
}

#[function_component(SubscriberExample)]
pub fn subscriber_example() -> Html {
    let websocket = use_worker_subscription::<WebsocketWorker>();

    // When the user click on the "Marco" button, we send a message to the backend
    // using the websocket. The backend will respond with "Polo" and we will display
    // it in the frontend.
    let on_marco = {
        let websocket = websocket.clone();
        Callback::from(move |_| {
            log::info!("Sending Marco to backend");
            websocket.send(FrontendMessage::Marco);
        })
    };

    html! {
        <div>
            <button onclick={on_marco}>{"Marco"}</button>
            <p>{format!("Backend says: {:?}", websocket.iter().last().cloned())}</p>
        </div>
    }
}

#[function_component(GaussExample)]
pub fn gauss_subscriber_example() -> Html {
    log::debug!("Rendering GaussExample component.");
    let last_message = use_state(|| None);

    let on_output = {
        let last_message = last_message.clone();
        move |message: BackendMessage| {
            log::info!("Got message from backend: {:?}", message);
            last_message.set(Some(message));
        }
    };

    let websocket = use_worker_bridge::<WebsocketWorker, _>(on_output);

    // When the user click on the "Euler" button, we send a message to the backend
    // using the websocket. The backend will respond with "Polo" and we will display
    // it in the frontend.
    let on_euler = {
        let websocket = websocket.clone();
        Callback::from(move |_| {
            log::info!("Sending Euler to backend");
            websocket.send(FrontendMessage::Euler);
        })
    };

    html! {
        <div>
            <button onclick={on_euler}>{"Euler"}</button>
            <p>{format!("Backend says: {:?}", *last_message)}</p>
        </div>
    }
}
