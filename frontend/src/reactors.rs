use gloo_net::websocket::{Message as GlooMessage, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use serde::{Serialize, Deserialize};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::Message;
use yew_agent::reactor::{reactor, ReactorScope};

#[derive(Debug, Serialize, Deserialize)]
pub enum WebsocketInput {
    SendMessage(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WebsocketOutput {
    ReceivedMessage(String),
}

#[reactor(WebsocketReactor)]
pub async fn websocket_reactor(mut scope: ReactorScope<WebsocketInput, WebsocketOutput>) {
    let ws = WebSocket::open("wss://echo.websocket.org").unwrap();

    let (mut write, mut read) = ws.split();

    // We setup a 
    spawn_local(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Text(data)) => {
                    log::debug!("from websocket: {}", data);
                }
                Ok(Message::Bytes(b)) => {
                    let decoded = std::str::from_utf8(&b);
                    if let Ok(val) = decoded {
                        log::debug!("from websocket: {}", val);
                    }
                }
                Err(e) => {
                    log::error!("ws: {:?}", e)
                }
            }
        }
        log::debug!("WebSocket Closed");
    });

    while let Some(input) = scope.next().await {
        match input {
            WebsocketInput::SendMessage(message) => {
                let msg = GlooMessage::Text(message);
                write.send(msg).await.unwrap();
            }
        }
    }
}