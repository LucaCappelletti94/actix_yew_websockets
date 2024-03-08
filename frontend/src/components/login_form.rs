use crate::stores::UserState;
use crate::worker::*;
use commons::messages::{BackendMessage, FrontendMessage};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::FormData;
use yew::prelude::*;
use yew_agent::prelude::*;
use yewdux::prelude::*;

pub struct LoginForm {
    websocket: WorkerBridgeHandle<WebsocketWorker<FrontendMessage, BackendMessage>>,
    user_state: Rc<UserState>,
    dispatch: Dispatch<UserState>,
}

#[derive(Debug, Clone)]
pub enum WebsocketMessages {
    Frontend(FrontendMessage),
    Backend(BackendMessage),
    State(Rc<UserState>),
}

impl Component for LoginForm {
    type Message = WebsocketMessages;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<UserState>::global()
            .subscribe(ctx.link().callback(WebsocketMessages::State));

        Self {
            websocket: ctx.link().bridge_worker(Callback::from({
                let link = ctx.link().clone();
                move |message: BackendMessage| {
                    link.send_message(WebsocketMessages::Backend(message));
                }
            })),
            user_state: dispatch.get(),
            dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebsocketMessages::Frontend(fm) => self.websocket.send(fm.into()),
            WebsocketMessages::Backend(bm) => match bm {
                BackendMessage::LoggedIn(user) => {
                    log::info!("User logged in: {:?}", user);
                    self.dispatch.reduce_mut(|state| {
                        state.user = Some(user);
                    });
                }
                _ => {}
            },
            WebsocketMessages::State(state) => {
                self.user_state = state;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_submit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            let user_name: String = e
                .target()
                .unwrap()
                .unchecked_into::<web_sys::HtmlFormElement>()
                .elements()
                .get_with_index(0)
                .unwrap()
                .unchecked_into::<web_sys::HtmlInputElement>()
                .value();
            WebsocketMessages::Frontend(FrontendMessage::Login(user_name))
        });

        html! {
            <form method="POST" onsubmit={on_submit}>
                <input type="text" placeholder="Username" />
                <button>{"Login"}</button>
            </form>
        }
    }
}
