//! Module providing the websocket messages used in the application.
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendMessage {
    Connect,
    Disconnect(Option<u16>),
    Marco,
    Euler
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BackendMessage {
    Polo,
    Gauss
}

impl From<BackendMessage> for Message {
    fn from(msg: BackendMessage) -> Self {
        match msg {
            BackendMessage::Polo => {
                Message::Text("Polo".to_string())
            },
            BackendMessage::Gauss => {
                Message::Text("Gauss".to_string())
            }
        }
    }
}

impl From<Message> for BackendMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(text) => {
                match text.as_str() {
                    "Polo" => BackendMessage::Polo,
                    "Gauss" => BackendMessage::Gauss,
                    _ => {
                        panic!("Unsupported message: {:?}", text)
                    }
                }
            }
            Message::Binary(_bin) => {
                unimplemented!(
                    "Converting a binary message to an BackendMessage is not supported"
                );
            }
        }
    }
}

impl From<FrontendMessage> for Message {
    fn from(msg: FrontendMessage) -> Self {
        match msg {
            FrontendMessage::Marco => {
                Message::Text("Marco".to_string())
            }
            FrontendMessage::Euler => {
                Message::Text("Euler".to_string())
            }
            FrontendMessage::Connect => {
                unimplemented!("Converting a connect message to a Message is not supported");
            }
            FrontendMessage::Disconnect(_code) => {
                unimplemented!("Converting a disconnect message to a Message is not supported");
            }
        }
    }
}

impl From<Message> for FrontendMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(text) => {
                match text.as_str() {
                    "Marco" => FrontendMessage::Marco,
                    "Euler" => FrontendMessage::Euler,
                    _ => {
                        panic!("Unsupported message: {:?}", text)
                    }
                }
            }
            Message::Binary(_bin) => {
                unimplemented!(
                    "Converting a binary message to an FrontendMessage is not supported"
                );
            }
        }
    }
}

/// When the feature `backend` is enabled, we implement the From trait for the
/// `Message` enum to convert from the `actix_web_actors::ws::Message` enum.
#[cfg(feature = "backend")]
impl From<actix_web_actors::ws::Message> for Message {
    fn from(msg: actix_web_actors::ws::Message) -> Self {
        match msg {
            actix_web_actors::ws::Message::Text(text) => Message::Text(text.to_string()),
            actix_web_actors::ws::Message::Binary(bin) => Message::Binary(bin.to_vec()),
            alternative => panic!("Unsupported message type: {:?}", alternative),
        }
    }
}

#[cfg(feature = "backend")]
impl From<actix_web_actors::ws::Message> for BackendMessage {
    fn from(msg: actix_web_actors::ws::Message) -> Self {
        let message: Message = msg.into();
        message.into()
    }
}

#[cfg(feature = "backend")]
impl From<actix_web_actors::ws::Message> for FrontendMessage {
    fn from(msg: actix_web_actors::ws::Message) -> Self {
        let message: Message = msg.into();
        message.into()
    }
}

/// When the feature `frontend` is enabled, we implement the From trait for the
/// `Message` enum to convert from the `gloo_net::websocket::Message` enum.
#[cfg(feature = "frontend")]
impl From<gloo_net::websocket::Message> for Message {
    fn from(msg: gloo_net::websocket::Message) -> Self {
        match msg {
            gloo_net::websocket::Message::Text(text) => Message::Text(text),
            gloo_net::websocket::Message::Bytes(bin) => Message::Binary(bin.to_vec()),
        }
    }
}

#[cfg(feature = "frontend")]
impl From<gloo_net::websocket::Message> for BackendMessage {
    fn from(msg: gloo_net::websocket::Message) -> Self {
        let message: Message = msg.into();
        message.into()
    }
}

#[cfg(feature = "frontend")]
impl From<gloo_net::websocket::Message> for FrontendMessage {
    fn from(msg: gloo_net::websocket::Message) -> Self {
        let message: Message = msg.into();
        message.into()
    }
}

#[cfg(feature = "frontend")]
impl From<Message> for gloo_net::websocket::Message {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(text) => gloo_net::websocket::Message::Text(text),
            Message::Binary(bin) => gloo_net::websocket::Message::Bytes(bin.into()),
        }
    }
}

#[cfg(feature = "frontend")]
impl From<FrontendMessage> for gloo_net::websocket::Message {
    fn from(msg: FrontendMessage) -> Self {
        let message: Message = msg.into();
        message.into()
    }
}

#[cfg(feature = "frontend")]
impl From<BackendMessage> for gloo_net::websocket::Message {
    fn from(msg: BackendMessage) -> Self {
        let message: Message = msg.into();
        message.into()
    }
}

#[cfg(feature = "backend")]
impl From<Message> for actix_web_actors::ws::Message {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(text) => actix_web_actors::ws::Message::Text(text.into()),
            Message::Binary(bin) => actix_web_actors::ws::Message::Binary(bin.into()),
        }
    }
}

#[cfg(feature = "backend")]
impl From<BackendMessage> for actix_web_actors::ws::Message {
    fn from(msg: BackendMessage) -> Self {
        let message: Message = msg.into();
        message.into()
    }
}

#[cfg(feature = "backend")]
impl From<FrontendMessage> for actix_web_actors::ws::Message {
    fn from(msg: FrontendMessage) -> Self {
        let message: Message = msg.into();
        message.into()
    }
}