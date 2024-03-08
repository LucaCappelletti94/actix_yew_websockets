use crate::worker::*;
use commons::messages::{BackendMessage, FrontendMessage};
use yew::prelude::*;
use yew_agent::prelude::*;

pub struct Comment {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
}

#[derive(Debug, Clone)]
pub enum WebsocketMessages {
    Frontend(FrontendMessage),
    Backend(BackendMessage),
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
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebsocketMessages::Frontend(fm) => self.websocket.send(fm.into()),
            WebsocketMessages::Backend(_bm) => {}
            WebsocketMessages::DeleteComment(comment) => {
                self.websocket.send(FrontendMessage::DeleteComment(comment));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // If the user is the author of the comment, they can delete it
        let delete_button = if ctx.props().comment.user_id == ctx.props().user.id {
            let comment = ctx.props().comment.clone();
            let on_delete_button = ctx
                .link()
                .callback(move |_| WebsocketMessages::DeleteComment(comment.clone()));
            html! {
                <button onclick={on_delete_button}>{"Delete"}</button>
            }
        } else {
            html! {}
        };

        html! {
            <li class="comment">
                <p>{&ctx.props().comment.body}</p>
                {delete_button}
            </li>
        }
    }
}
