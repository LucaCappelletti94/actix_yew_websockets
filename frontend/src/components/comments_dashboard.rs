use crate::worker::*;
use commons::messages::{BackendMessage, FrontendMessage};
use yew::prelude::*;
use yew_agent::prelude::*;
use wasm_bindgen::JsCast;
use crate::components::CommentPopup;
use crate::components::Comment;

pub struct CommentsDashboard {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    comments: Vec<commons::comments::Comment>,
}

#[derive(Debug, Clone)]
pub enum WebsocketMessages {
    Frontend(FrontendMessage),
    Backend(BackendMessage),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CommentsDashboardProps {
    pub user: commons::prelude::User,
}

impl Component for CommentsDashboard {
    type Message = WebsocketMessages;
    type Properties = CommentsDashboardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(WebsocketMessages::Backend(message));
                }
            })),
            comments: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebsocketMessages::Frontend(fm) => self.websocket.send(fm.into()),
            WebsocketMessages::Backend(bm) => match bm {
                BackendMessage::NewComment(comment) => {
                    log::info!("New comment: {:?}", comment);
                    self.comments.push(comment);
                }
                BackendMessage::DeletedComment(comment) => {
                    log::info!("Deleted comment: {:?}", comment);
                    self.comments.retain(|c| c.id != comment.id);
                }
                _ => {}
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user = ctx.props().user.clone();
        let on_submit_comment = ctx.link().callback(move |event: SubmitEvent| {
            event.prevent_default();
            let comment = event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlFormElement>()
                .unwrap()
                .elements()
                .named_item("comment")
                .unwrap()
                .dyn_into::<web_sys::HtmlTextAreaElement>()
                .unwrap()
                .value();
            WebsocketMessages::Frontend(FrontendMessage::InsertComment((user.clone(), comment)))
        });

        let comments = self
            .comments
            .iter()
            .map(|comment| {
                html! {
                    <Comment user={ctx.props().user.clone()} comment={comment.clone()} />
                }
            })
            .collect::<Html>();

        html! {
            <div class="comments-dashboard">
                <CommentPopup/>
                <form onsubmit={on_submit_comment}>
                    <textarea name="comment" placeholder="Write a comment..." required=true />
                    <button type="submit">{"Submit"}</button>
                </form>
                <ul class="comments">
                    {comments}
                </ul>
            </div>
        }
    }
}
