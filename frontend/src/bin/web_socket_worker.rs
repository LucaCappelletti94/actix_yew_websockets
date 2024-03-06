use yew_agent::Registrable;
use frontend::worker::WebsocketWorker;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    WebsocketWorker::registrar().register();
}