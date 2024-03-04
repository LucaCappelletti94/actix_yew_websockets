use actix_files::{Files, NamedFile};
use actix_web::get;
use actix_web::HttpResponse;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer, Responder};
mod ws;

#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[get("/ws")]
async fn start_websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    actix_web_actors::ws::start(ws::WebSocket, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(start_websocket)
            .service(Files::new("/static", "./static"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
