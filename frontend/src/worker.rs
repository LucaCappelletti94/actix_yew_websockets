use commons::messages::{BackendMessage, FrontendMessage};
use futures::{Future, SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use core::panic;
use std::collections::HashSet;
use wasm_bindgen::UnwrapThrowExt;
use yew::platform::spawn_local;
use yew_agent::worker::{HandlerId, WorkerScope};
use yew_agent::{
    reactor::{reactor, Reactor, ReactorScope},
    worker::Worker,
};

const NOMINAL_CLOSURE_CODE: u16 = 1000;

pub struct WebsocketWorker {
    subscribers: HashSet<HandlerId>,
    sender: Option<futures::channel::mpsc::Sender<FrontendMessage>>,
}

#[derive(Clone, Debug)]
pub enum InternalMessage {
    Backend(BackendMessage),
    Disconnect(Option<u16>),
}

impl Worker for WebsocketWorker {
    type Message = InternalMessage;
    type Input = FrontendMessage;
    type Output = BackendMessage;

    fn create(scope: &yew_agent::prelude::WorkerScope<Self>) -> Self {
        let websocket = WebSocket::open("ws://localhost:8080/ws").unwrap_throw();
        let (mut write, mut read) = websocket.split();

        let (sender, mut receiver) = futures::channel::mpsc::channel::<FrontendMessage>(1000);

        spawn_local(async move {
            while let Some(frontend_message) = receiver.next().await {
                log::debug!("got event from channel! {:?}", frontend_message);
                write.send(frontend_message.into()).await.unwrap();
            }
        });

        {
            let scope = scope.clone();
            spawn_local(async move {
                while let Some(backend_message) = read.next().await {
                    match backend_message {
                        Ok(message) => {
                            log::debug!("Got message from websocket: {:?}", message);
                            scope.send_message(InternalMessage::Backend(message.into()));
                        }
                        Err(err) => {
                            log::error!("Error reading from websocket: {:?}", err);
                        }
                    }
                }
                log::debug!("WebSocket Closed");
            });
        }

        Self {
            subscribers: HashSet::new(),
            sender: Some(sender),
        }
    }

    fn update(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        internal_message: Self::Message,
    ) {
        match internal_message {
            InternalMessage::Backend(backend_message) => {
                log::debug!("Got message from websocket: {:?}", backend_message);
                // We log the current number of subscribers
                log::debug!(
                    "Current number of subscribers: {:?}",
                    self.subscribers.len()
                );
                for sub in &self.subscribers {
                    log::debug!(
                        "Sending message to subscriber {:?}: {:?}",
                        sub,
                        backend_message
                    );
                    scope.respond(*sub, backend_message.clone());
                }
            }
            InternalMessage::Disconnect(closure_code) => {
                if let Some(mut sender) = self.sender.take() {
                    spawn_local(async move {
                        sender.close().await.unwrap_throw();
                    });
                }
            }
        }
    }

    fn connected(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        id: yew_agent::worker::HandlerId,
    ) {
        // Chweck if the subscriber is connected
        if self.subscribers.contains(&id) {
            log::debug!("Subscriber already connected: {:?}", id);
            return;
        }

        log::debug!("Connected to subscriber: {:?}", id);
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, scope: &yew_agent::prelude::WorkerScope<Self>, id: HandlerId) {
        log::debug!("Disconnected from subscriber: {:?}", id);
        self.subscribers.remove(&id);
    }

    fn destroy(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        _destruct: yew_agent::worker::WorkerDestroyHandle<Self>,
    ) {
        log::debug!("Destroying websocket worker");
        scope.send_message(InternalMessage::Disconnect(Some(NOMINAL_CLOSURE_CODE)));
    }

    fn received(
        &mut self,
        scope: &yew_agent::prelude::WorkerScope<Self>,
        frontend_message: Self::Input,
        id: HandlerId,
    ) {
        if let Some(sender) = &mut self.sender {
            log::debug!("Sending message to websocket: {:?}", frontend_message);
            match sender.try_send(frontend_message) {
                Ok(()) => {
                    log::debug!("Sent message to websocket");
                }
                Err(err) => {
                    log::error!("Error sending message to websocket: {:?}", err);
                }
            }
        }
    }
}