use crate::worker::*;
use commons::{
    comments::Comment,
    messages::{BackendMessage, FrontendMessage},
};
use yew::prelude::*;
use yew_agent::prelude::*;

pub struct CommentPopup {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    comment: Option<Comment>,
}

#[derive(Debug, Clone)]
pub enum WebsocketMessages {
    Frontend(FrontendMessage),
    Backend(BackendMessage),
    CloseCommentPopup
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebsocketMessages::Frontend(fm) => self.websocket.send(fm.into()),
            WebsocketMessages::Backend(bm) => match bm {
                BackendMessage::InsertedComment(comment) => {
                    log::info!("Inserted comment: {:?}", comment);
                    self.comment = Some(comment);
                }
                _ => {}
            },
            WebsocketMessages::CloseCommentPopup => {
                self.comment = None;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_close_button = ctx.link().callback(|_| WebsocketMessages::CloseCommentPopup);

        if let Some(comment) = &self.comment {
            html! {
                <div class="comment-popup">
                    <div class="comment-popup-content">
                        <span class="close" onclick={on_close_button}>{"\u{00D7}"}</span>
                        <h2>{"Inserted Comment"}</h2>
                        <p>{&comment.body}</p>
                    </div>
                </div>
            }
        } else {
            html! {<></>}
        }
    }
}
