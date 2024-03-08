use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::get;
use actix_web::http::header;
use actix_web::HttpResponse;
use actix_web::{middleware::Logger, web, App, Error, HttpRequest, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, Pool as DieselPool};
use sqlx::{postgres::PgPoolOptions, Pool as SQLxPool, Postgres};
mod models;
mod schema;
mod channel_listeners;
mod ws;

// #[get("/")]
// async fn index() -> impl Responder {
//     NamedFile::open_async("index.html").await.unwrap()
// }

#[get("/ws")]
async fn start_websocket(
    req: HttpRequest,
    stream: web::Payload,
    diesel_pool: web::Data<DSDBPool>,
    sqlx_pool: web::Data<SQLxPool<Postgres>>,
) -> Result<HttpResponse, Error> {
    let diesel_conn = match diesel_pool.get() {
        Ok(diesel_conn) => diesel_conn,
        Err(e) => {
            log::error!("🔥 Error connecting to the database: {}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let sqlx_pool = sqlx_pool.get_ref().clone();

    actix_web_actors::ws::start(ws::WebSocket::new(diesel_conn, sqlx_pool), &req, stream)
}

pub(crate) type DSDBPool = DieselPool<ConnectionManager<PgConnection>>;
pub(crate) type DieselConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let diesel_pool: DSDBPool = match r2d2::Pool::builder()
        // We set the maximum number of connections in the pool to 10
        .max_size(10)
        .build(manager)
    {
        Ok(client) => {
            log::info!("✅Connection to the database is successful!");
            client
        }
        Err(e) => {
            log::error!("🔥 Error connecting to the database: {}", e);
            std::process::exit(1);
        }
    };

    let sqlx_pool: SQLxPool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

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
            // pass in the Diesel database pool to all routes
            .app_data(web::Data::new(diesel_pool.clone()))
            // pass in the SQLx database pool to all routes
            .app_data(web::Data::new(sqlx_pool.clone()))
            // .service(index)
            .service(start_websocket)
            // .service(Files::new("/static", "./static"))
            .wrap(Logger::default())
    })
    .workers(2)
    .bind(("localhost", 8080))?
    .run()
    .await?;

    Ok(())
}
