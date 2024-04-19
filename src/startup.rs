use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use crate::routes::*;
use sqlx::PgPool;

use tracing_actix_web::TracingLogger;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub async fn run(listener: TcpListener, 
                connection_pool: PgPool) -> Result<Server, std::io::Error> {

    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

