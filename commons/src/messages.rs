//! Module providing the websocket messages used in the application.
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendMessage {
    Connect,
    Disconnect(Option<u16>),
    StartCounter,
    StopCounter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BackendMessage {
    CurrentCount(u32),
    StoppedCounter,
}

impl From<BackendMessage> for Message {
    fn from(msg: BackendMessage) -> Self {
        Message::Text(serde_json::to_string(&msg).expect("Failed to serialize message"))
    }
}

impl From<Message> for BackendMessage {
    fn from(msg: Message) -> Self {
        log::info!("Converting message to backend message: {:?}", msg);
        match msg {
            Message::Text(text) => match serde_json::from_str(&text) {
                Ok(backend_message) => backend_message,
                Err(err) => {
                    log::error!("Failed to deserialize message: {:?}", err);
                    panic!("Failed to deserialize message: {:?}", err);
                }
            },
            Message::Binary(_bin) => {
                log::error!("Converting a binary message to an BackendMessage is not supported");
                unimplemented!("Converting a binary message to an BackendMessage is not supported");
            }
        }
    }
}

impl From<FrontendMessage> for Message {
    fn from(msg: FrontendMessage) -> Self {
        Message::Text(serde_json::to_string(&msg).expect("Failed to serialize message"))
    }
}

impl From<Message> for FrontendMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(text) => match serde_json::from_str(&text) {
                Ok(frontend_message) => frontend_message,
                Err(err) => {
                    log::error!("Failed to deserialize message: {:?}", err);
                    panic!("Failed to deserialize message: {:?}", err);
                }
            },
            Message::Binary(_bin) => {
                log::error!("Converting a binary message to an FrontendMessage is not supported");
                unimplemented!(
                    "Converting a binary message to an FrontendMessage is not supported"
                );
            }
        }
    }
}

/// When the feature `backend` is enabled, we implement the From trait for the
/// ByteString enum.
#[cfg(feature = "backend")]
impl Into<bytestring::ByteString> for BackendMessage {
    fn into(self) -> bytestring::ByteString {
        match self.into() {
            Message::Text(text) => {
                text.into()
            },
            Message::Binary(_bin) => {
                log::error!("Converting a binary message to a ByteString is not supported");
                panic!("Converting a binary message to a ByteString is not supported")
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
            alternative => {
                log::error!("Unsupported message type: {:?}", alternative);
                panic!("Unsupported message type: {:?}", alternative);
            }
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
        log::info!("Converting message to backend message: {:?}", msg);
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
