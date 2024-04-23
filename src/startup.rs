use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use crate::routes::*;
use sqlx::PgPool;

use tracing_actix_web::TracingLogger;
use std::net::TcpListener;




pub async fn run(listener: TcpListener, 
                connection_pool: PgPool) -> Result<Server, std::io::Error> {

    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/play", web::get().to(play))
            .route("/game", web::get().to(read_game_data))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

