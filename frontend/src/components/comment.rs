use crate::worker::*;
use commons::messages::{BackendMessage, FrontendMessage};
use yew::prelude::*;
use yew_agent::prelude::*;
use gloo::timers::callback::Timeout;

pub struct Comment {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    deleting: Option<Timeout>
}

#[derive(Debug, Clone)]
pub enum WebsocketMessages {
    Frontend(FrontendMessage),
    Backend(BackendMessage),
    StartDeleteComment(commons::comments::Comment),
    DeleteComment(commons::comments::Comment),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CommentProps {
    pub user: commons::users::User,
    pub comment: commons::comments::Comment,
}

impl Component for Comment {
    type Message = WebsocketMessages;
    type Properties = CommentProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(WebsocketMessages::Backend(message));
                }
            })),
            deleting: None
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebsocketMessages::Frontend(fm) => self.websocket.send(fm.into()),
            WebsocketMessages::Backend(_bm) => {}
            WebsocketMessages::StartDeleteComment(comment) => {
                let link = ctx.link().clone();
                self.deleting = Some(Timeout::new(1000, move || {
                    link.send_message(WebsocketMessages::DeleteComment(comment));
                }));
            }
            WebsocketMessages::DeleteComment(comment) => {
                if let Some(timeout) = self.deleting.take() {
                    timeout.cancel();
                }
                self.websocket.send(FrontendMessage::DeleteComment(comment));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // If the user is the author of the comment, they can delete it
        let delete_button = if ctx.props().comment.user_id == ctx.props().user.id {
            let comment = ctx.props().comment.clone();
            let on_delete_button = ctx.link().callback(move |event: SubmitEvent| {
                event.prevent_default();
                WebsocketMessages::StartDeleteComment(comment.clone())
            });
            html! {
                <form method="DELETE" onsubmit={on_delete_button}>
                    <button type="submit">{"Delete"}</button>
                </form>
            }
        } else {
            html! {}
        };

        let classes = if self.deleting.is_some() {
            "comment deleting"
        } else {
            "comment"
        };

        html! {
            <li class={classes}>
                <p>{&ctx.props().comment.body}</p>
                {delete_button}
            </li>
        }
    }
}
