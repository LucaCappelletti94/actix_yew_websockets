use yew_agent::Registrable;
use frontend::worker::WebsocketWorker;
use commons::messages::{FrontendMessage, BackendMessage};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    WebsocketWorker::<FrontendMessage, BackendMessage>::registrar().register();
}