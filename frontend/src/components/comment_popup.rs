use crate::worker::*;
use commons::{
    comments::Comment,
    messages::{BackendMessage, FrontendMessage},
};
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yew_agent::prelude::*;

pub struct CommentPopup {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    comment: Option<Comment>,
    hiding: Option<Timeout>,
    before_hiding: Option<Timeout>,
}

#[derive(Debug, Clone)]
pub enum WebsocketMessages {
    Frontend(FrontendMessage),
    Backend(BackendMessage),
    CloseCommentPopup,
    RemoveComment,
}

impl Component for CommentPopup {
    type Message = WebsocketMessages;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(WebsocketMessages::Backend(message));
                }
            })),
            comment: None,
            hiding: None,
            before_hiding: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebsocketMessages::Frontend(fm) => self.websocket.send(fm.into()),
            WebsocketMessages::Backend(bm) => match bm {
                BackendMessage::InsertedComment(comment) => {
                    log::info!("Inserted comment: {:?}", comment);
                    self.comment = Some(comment);
                    let link = ctx.link().clone();
                    if let Some(timeout) = self.before_hiding.take() {
                        timeout.cancel();
                    }
                    if let Some(timeout) = self.hiding.take() {
                        timeout.cancel();
                    }
                    self.before_hiding = Some(Timeout::new(5000, move || {
                        link.send_message(WebsocketMessages::CloseCommentPopup);
                    }));
                }
                _ => {}
            },
            WebsocketMessages::CloseCommentPopup => {
                if let Some(timeout) = self.before_hiding.take() {
                    timeout.cancel();
                }
                let link = ctx.link().clone();
                self.hiding = Some(Timeout::new(1000, move || {
                    link.send_message(WebsocketMessages::RemoveComment);
                }));
            }
            WebsocketMessages::RemoveComment => {
                self.comment = None;
                if let Some(timeout) = self.hiding.take() {
                    timeout.cancel();
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_close_button = ctx
            .link()
            .callback(|_| WebsocketMessages::CloseCommentPopup);

        if let Some(comment) = &self.comment {
            let classes = if self.hiding.is_some() {
                "comment-popup hiding"
            } else {
                "comment-popup"
            };

            html! {
                <div class={classes}>
                    <span class="close" onclick={on_close_button}>{"\u{00D7}"}</span>
                    <h4>{"Inserted comment"}</h4>
                    <p>{&comment.body}</p>
                </div>
            }
        } else {
            html! {<></>}
        }
    }
}
