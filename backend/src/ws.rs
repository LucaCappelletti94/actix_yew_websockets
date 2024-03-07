//! Websocket backend
use actix::ActorContext;
use actix::AsyncContext;
use actix::SpawnHandle;
use actix::WrapFuture;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use commons::messages::{BackendMessage, FrontendMessage};
use std::time::Duration;

use crate::Conn;

pub struct WebSocket {
    pg_handlers: Vec<SpawnHandle>,
    conn: Conn,
}

impl WebSocket {
    pub fn new(conn: Conn) -> Self {
        Self {
            pg_handlers: vec![],
            conn,
        }
    }
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
                    FrontendMessage::Login(username) => {
                        // We insert the user into the database
                        let new_user = crate::models::NewUser {
                            username: username.clone(),
                        };

                        match new_user.insert(&mut self.conn) {
                            Ok(user) => {
                                ctx.binary(BackendMessage::LoggedIn(user.into()));
                            }
                            Err(err) => {
                                log::error!("Error inserting user: {:?}", err);
                            }
                        }

                        self.pg_handlers
                            .push(ctx.spawn(async move {
                                // We check whether there are any news from
                                // the postgres channel "comment_added"
                                
                                todo!("Check for new comments");

                            }.into_actor(self)));
                    }
                    FrontendMessage::InsertComment((user, comment_text)) => {
                        let new_comment = crate::models::NewComment {
                            user_id: user.id,
                            body: comment_text,
                        };

                        match new_comment.insert(&mut self.conn) {
                            Ok(comment) => {
                                ctx.binary(BackendMessage::NewComment(comment.into()));
                            }
                            Err(err) => {
                                log::error!("Error inserting comment: {:?}", err);
                            }
                        }
                    }
                    FrontendMessage::DeleteComment(comment) => {
                        let comment: crate::models::Comment = comment.into();
                        match comment.delete(&mut self.conn) {
                            Ok(_) => {
                                // We could trigger the event here,
                                // but we want to handle it separately in the
                                // pg_notify handler
                            }
                            Err(err) => {
                                log::error!("Error deleting comment: {:?}", err);
                            }
                        }
                    }
                    FrontendMessage::Close(code) => {
                        ctx.stop();
                    }
                    _ => {
                        log::error!("Unhandled message: {:?}", frontend_message);
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
