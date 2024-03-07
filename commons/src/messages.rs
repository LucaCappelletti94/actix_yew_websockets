//! Module providing the websocket messages used in the application.
use serde::{Deserialize, Serialize};

use crate::prelude::{Comment, User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloseReason {
    code: u16,
    reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendMessage {
    Close(Option<CloseReason>),
    Login(String),
    InsertComment((User, String)),
    DeleteComment(Comment)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BackendMessage {
    LoggedIn(User),
    NewComment(Comment),
    InsertedComment(Comment),
    Comments(Vec<Comment>),
    DeletedComment(Comment),
}

#[cfg(feature = "backend")]
impl From<BackendMessage> for bytes::Bytes {
    fn from(msg: BackendMessage) -> Self {
        bincode::serialize(&msg).unwrap().into()
    }
}

#[cfg(feature = "backend")]
impl From<actix_web_actors::ws::Message> for FrontendMessage {
    fn from(actix_message: actix_web_actors::ws::Message) -> Self {
        match actix_message {
            actix_web_actors::ws::Message::Text(text) => {
                log::error!("Unexpected text message from backend: {:?}", text);
                unreachable!("Unexpected text message from backend");
            }
            actix_web_actors::ws::Message::Binary(bin) => bincode::deserialize(&bin).unwrap(),
            actix_web_actors::ws::Message::Ping(_) => {
                log::error!("Unexpected ping message from backend");
                unreachable!("Unexpected ping message from backend");
            }
            actix_web_actors::ws::Message::Pong(_) => {
                log::error!("Unexpected pong message from backend");
                unreachable!("Unexpected pong message from backend");
            }
            actix_web_actors::ws::Message::Close(reason) => {
                FrontendMessage::Close(reason.map(|r: actix_web_actors::ws::CloseReason| {
                    CloseReason {
                        code: r.code.into(),
                        reason: r.description,
                    }
                }))
            }
            actix_web_actors::ws::Message::Continuation(_) => {
                log::error!("Unexpected continuation message from backend");
                unreachable!("Unexpected continuation message from backend");
            }
            actix_web_actors::ws::Message::Nop => {
                log::error!("Unexpected nop message from backend");
                unreachable!("Unexpected nop message from backend");
            }
        }
    }
}

#[cfg(feature = "frontend")]
impl From<gloo_net::websocket::Message> for BackendMessage {
    fn from(msg: gloo_net::websocket::Message) -> Self {
        match msg {
            gloo_net::websocket::Message::Text(text) => {
                log::error!("Unexpected text message from frontend: {:?}", text);
                unreachable!("Unexpected text message from frontend");
            }
            gloo_net::websocket::Message::Bytes(bin) => bincode::deserialize(&bin).unwrap(),
        }
    }
}

#[cfg(feature = "frontend")]
impl From<FrontendMessage> for gloo_net::websocket::Message {
    fn from(msg: FrontendMessage) -> Self {
        gloo_net::websocket::Message::Bytes(bincode::serialize(&msg).unwrap())
    }
}
