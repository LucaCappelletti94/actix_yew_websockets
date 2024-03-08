//! Websocket backend
use actix::ActorContext;
use actix::AsyncContext;
use actix::SpawnHandle;
use actix::WrapFuture;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use commons::messages::{BackendMessage, FrontendMessage};
use sqlx::{Pool as SQLxPool, Postgres};

use crate::channel_listeners::*;
use crate::DieselConn;

pub struct WebSocket {
    pg_handlers: Vec<SpawnHandle>,
    diesel: DieselConn,
    sqlx: SQLxPool<Postgres>,
}

impl WebSocket {
    pub fn new(diesel: DieselConn, sqlx: SQLxPool<Postgres>) -> Self {
        Self {
            pg_handlers: vec![],
            diesel,
            sqlx,
        }
    }
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl actix::Handler<BackendMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: BackendMessage, ctx: &mut Self::Context) {
        ctx.binary(msg);
    }
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

                        match new_user.insert_or_get(&mut self.diesel) {
                            Ok(user) => {
                                ctx.binary(BackendMessage::LoggedIn(user.clone().into()));

                                let recipient = ctx.address();
                                let sqlx = self.sqlx.clone();
                                self.pg_handlers.push(
                                    ctx.spawn(
                                        async move {
                                            let _ = start_listening(
                                                &sqlx,
                                                CommentsChannel,
                                                |payload: CommentsPayload| {
                                                    match payload.action_type {
                                                        ActionType::INSERT => {
                                                            recipient.do_send(
                                                                BackendMessage::NewComment(
                                                                    payload.into(),
                                                                ),
                                                            );
                                                        }
                                                        ActionType::UPDATE => {
                                                            recipient.do_send(
                                                                BackendMessage::UpdatedComment(
                                                                    payload.into(),
                                                                ),
                                                            );
                                                        }
                                                        ActionType::DELETE => {
                                                            recipient.do_send(
                                                                BackendMessage::DeletedComment(
                                                                    payload.into(),
                                                                ),
                                                            );
                                                        }
                                                    };
                                                },
                                            )
                                            .await;
                                        }
                                        .into_actor(self),
                                    ),
                                );

                                let recipient = ctx.address();
                                let sqlx = self.sqlx.clone();
                                self.pg_handlers.push(
                                    ctx.spawn(
                                        async move {
                                            let _ = start_listening(
                                                &sqlx,
                                                CommentsUserChannel::new(user.into()),
                                                |payload: CommentsPayload| {
                                                    match payload.action_type {
                                                        ActionType::INSERT => {
                                                            recipient.do_send(
                                                                BackendMessage::InsertedComment(
                                                                    payload.into(),
                                                                ),
                                                            );
                                                        }
                                                        _ => {}
                                                    };
                                                },
                                            )
                                            .await;
                                        }
                                        .into_actor(self),
                                    ),
                                );
                            }
                            Err(err) => {
                                log::error!("Error inserting user: {:?}", err);
                            }
                        };
                    }
                    FrontendMessage::InsertComment((user, comment_text)) => {
                        let new_comment = crate::models::NewComment {
                            user_id: user.id,
                            body: comment_text,
                        };

                        match new_comment.insert(&mut self.diesel) {
                            Ok(_) => {}
                            Err(err) => {
                                log::error!("Error inserting comment: {:?}", err);
                            }
                        }
                    }
                    FrontendMessage::DeleteComment(comment) => {
                        let comment: crate::models::Comment = comment.into();
                        match comment.delete(&mut self.diesel) {
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
