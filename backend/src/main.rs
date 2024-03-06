use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::get;
use actix_web::http::header;
use actix_web::HttpResponse;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer};
mod ws;

// #[get("/")]
// async fn index() -> impl Responder {
//     NamedFile::open_async("index.html").await.unwrap()
// }

#[get("/ws")]
async fn start_websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    actix_web_actors::ws::start(ws::WebSocket, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        // Since in the development we are currently using Trunk, we need to
        // support CROSS ORIGIN RESOURCE SHARING (CORS) for the frontend
        // to be able to connect to the backend.

        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            // We add support for CORS requests
            .wrap(cors)
            // .service(index)
            .service(start_websocket)
            // .service(Files::new("/static", "./static"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("localhost", 8080))?
    .run()
    .await
}
