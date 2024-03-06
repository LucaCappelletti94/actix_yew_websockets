use crate::worker::*;
use commons::messages::{BackendMessage, FrontendMessage};
use yew::prelude::*;
use yew_agent::prelude::*;

pub struct Counter {
    websocket: WorkerBridgeHandle<WebsocketWorker>,
    message: Option<BackendMessage>,
    counting: bool,
}

#[derive(Debug, Clone)]
pub enum WebsocketMessages {
    Frontend(FrontendMessage),
    Backend(BackendMessage),
}

impl From<FrontendMessage> for WebsocketMessages {
    fn from(msg: FrontendMessage) -> Self {
        WebsocketMessages::Frontend(msg)
    }
}

impl From<BackendMessage> for WebsocketMessages {
    fn from(msg: BackendMessage) -> Self {
        WebsocketMessages::Backend(msg)
    }
}

impl Component for Counter {
    type Message = WebsocketMessages;
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
            counting: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebsocketMessages::Frontend(msg) => {
                log::info!("Sending message to backend: {:?}", msg);
                match msg {
                    FrontendMessage::StartCounter => {
                        self.counting = true;
                    }
                    FrontendMessage::StopCounter => {
                        self.counting = false;
                    }
                    _ => {}
                }
                self.message = None;
                self.websocket.send(msg.into())
            }
            WebsocketMessages::Backend(msg) => {
                log::info!("Got message from backend: {:?}", msg);
                self.message = Some(msg);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let toggle_counting = {
            let link = ctx.link().clone();
            let counting = self.counting;
            move |_| {
                if counting {
                    link.send_message(FrontendMessage::StopCounter);
                } else {
                    link.send_message(FrontendMessage::StartCounter);
                }
            }
        };

        html! {
            <div>
                <button onclick={toggle_counting}>{
                    if self.counting {
                        "Stop counting"
                    } else {
                        "Start counting"
                    }
                }</button>
                <p>{format!("Backend says: {:?}", self.message)}</p>
            </div>
        }
    }
}
