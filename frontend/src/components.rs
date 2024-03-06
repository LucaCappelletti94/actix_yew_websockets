//! This is the main file for the frontend. It will render the App component.
use crate::worker::*;
use commons::messages::{BackendMessage, FrontendMessage};
use log::info;
use yew::{callback, prelude::*};
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

pub struct GaussExample {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    message: Option<BackendMessage>,
}

#[derive(Debug, Clone)]
pub enum GaussMessages {
    Frontend(FrontendMessage),
    Backend(BackendMessage),
}

impl From<FrontendMessage> for GaussMessages {
    fn from(msg: FrontendMessage) -> Self {
        GaussMessages::Frontend(msg)
    }
}

impl From<BackendMessage> for GaussMessages {
    fn from(msg: BackendMessage) -> Self {
        GaussMessages::Backend(msg)
    }
}

impl Component for GaussExample {
    type Message = GaussMessages;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let on_output = {
            let link = ctx.link().clone();
            move |message: BackendMessage| {
                log::info!("Got message from backend: {:?}", message);
                link.send_message(message);
            }
        };

        let websocket = ctx.link().bridge_worker(Callback::from(on_output));

        Self {
            websocket,
            message: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GaussMessages::Frontend(msg) => {
                log::info!("Sending message to backend: {:?}", msg);
                self.message = None;
                self.websocket.send(
                    msg.into()
                )
            }
            GaussMessages::Backend(msg) => {
                log::info!("Got message from backend: {:?}", msg);
                self.message = Some(msg);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // When the user click on the "Euler" button, we send a message to the backend
        // using the websocket. The backend will respond with "Polo" and we will display
        // it in the frontend.
        let on_euler = ctx.link().callback(|_| {
            log::info!("Sending Euler to backend");
            FrontendMessage::Euler
        });

        html! {
            <div>
                <button onclick={on_euler}>{"Euler"}</button>
                <p>{format!("Backend says: {:?}", self.message)}</p>
            </div>
        }
    
    }
}


// #[function_component(GaussExample)]
// pub fn gauss_subscriber_example() -> Html {
//     log::debug!("Rendering GaussExample component.");
//     let last_message = use_state(|| None);

//     let on_output = {
//         let last_message = last_message.clone();
//         move |message: BackendMessage| {
//             log::info!("Got message from backend: {:?}", message);
//             last_message.set(Some(message));
//         }
//     };

//     let websocket = use_worker_bridge::<WebsocketWorker, _>(on_output);

//     // When the user click on the "Euler" button, we send a message to the backend
//     // using the websocket. The backend will respond with "Polo" and we will display
//     // it in the frontend.
//     let on_euler = {
//         let websocket = websocket.clone();
//         Callback::from(move |_| {
//             log::info!("Sending Euler to backend");
//             websocket.send(FrontendMessage::Euler);
//         })
//     };

//     html! {
//         <div>
//             <button onclick={on_euler}>{"Euler"}</button>
//             <p>{format!("Backend says: {:?}", *last_message)}</p>
//         </div>
//     }
// }
