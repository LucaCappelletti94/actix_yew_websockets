//! Websocket backend
use actix::ActorContext;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use commons::messages::{BackendMessage, FrontendMessage};

pub struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(msg) => {
                let frontend_message: FrontendMessage = msg.into();
                match frontend_message {
                    FrontendMessage::Marco => {
                        log::info!("Received Marco from frontend");
                        if let ws::Message::Text(byte_string_msg) = BackendMessage::Polo.into() {
                            ctx.text(byte_string_msg);
                        }
                    }
                    FrontendMessage::Euler => {
                        log::info!("Received Euler from frontend");
                        if let ws::Message::Text(byte_string_msg) = BackendMessage::Gauss.into() {
                            ctx.text(byte_string_msg);
                        }
                    }
                    FrontendMessage::Connect => {
                        unreachable!("Connect message should not be sent from the frontend");
                    }
                    FrontendMessage::Disconnect(code) => {
                        ctx.stop();
                    }
                }
            }
            Err(err) => {
                log::error!("Error reading from WebSocket: {:?}", err);
                ctx.stop();
            }
        }
    }
}
