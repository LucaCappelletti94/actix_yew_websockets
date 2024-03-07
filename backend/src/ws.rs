//! Websocket backend
use actix::ActorContext;
use actix::AsyncContext;
use actix::SpawnHandle;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use commons::messages::{BackendMessage, FrontendMessage};
use std::time::Duration;

#[derive(Default)]
pub struct WebSocket {
    counter: u32,
    counting: bool,
    counting_handler: Option<SpawnHandle>,
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(msg) => {
                log::info!("Got message from WebSocket: {:?}", msg);
                let frontend_message: FrontendMessage = msg.into();
                match frontend_message {
                    FrontendMessage::StartCounter => {
                        log::info!("Starting counter");
                        self.counting = true;
                        self.counting_handler =
                            Some(ctx.run_interval(Duration::from_secs(1), |act, ctx| {
                                // check client heartbeats
                                if !act.counting {
                                    // don't try to send a ping
                                    log::info!("Stopping counter");
                                    return;
                                }

                                act.counter += 1;

                                ctx.binary(BackendMessage::CurrentCount(act.counter))
                            }));
                    }
                    FrontendMessage::StopCounter => {
                        self.counting = false;
                        self.counting_handler.take().map(|h| ctx.cancel_future(h));
                        ctx.binary(BackendMessage::StoppedCounter);
                    }
                    FrontendMessage::Connect => {
                        unreachable!("Connect message should not be sent from the frontend");
                    }
                    FrontendMessage::Close(code) => {
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
