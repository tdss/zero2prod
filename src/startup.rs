use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, ResponseError};
use uuid::Uuid;
use crate::routes::*;
use sqlx::PgPool;
use serde::{Serialize,Deserialize};

use tracing_actix_web::TracingLogger;
use std::array;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


#[derive(Serialize, Deserialize)]
struct PlayerData {
    username: String,
    email: String
}

#[derive(Serialize)]
struct PlayerResponseData {
    username: String,
    id: uuid::Uuid
}

async fn play(req: web::Query<PlayerData>) -> actix_web::Result<impl Responder> {
    let obj = PlayerResponseData {
        username: req.username.to_string(),
        id: uuid::Uuid::new_v4()
    };
    Ok(web::Json(obj))
}



#[derive(Deserialize)]
struct GameRequestData {
    game_name: String,
}

#[derive(Serialize)]
struct GameResponseData {
    game_name: String,
    game_description: String,
    game_reward: String,
    game_tnc: String,
    id: uuid::Uuid,
    time_to_cooldown: i32,
    chances_left: i32,
    cooldown_to_win_time: i32,
    current_winner: PlayerResponseData,
    previous_winners: Vec<PlayerResponseData>
}

async fn readGameData(req: web::Query<GameRequestData>) -> actix_web::Result<impl Responder> {
    let obj = GameResponseData {
        game_name: req.game_name.to_string(),
        id: uuid::Uuid::new_v4(),
        game_description: "This is cool game".to_string(),
        game_reward: "$40".to_string(),
        game_tnc: "All rights reserved. Nothing guaranteed. It is just a game".to_string(),
        time_to_cooldown: 3600*1_000,
        chances_left: 10,
        cooldown_to_win_time: 4*3600*1_000,
        current_winner: PlayerResponseData {
            username: "Winner".to_string(),
            id: Uuid::new_v4()
        },
        previous_winners: vec![
            PlayerResponseData{username: "First Loser".to_string(), id: Uuid::new_v4()}, 
        ]
    };
    Ok(web::Json(obj))
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
            .route("/play", web::get().to(play))
            .route("/game", web::get().to(readGameData))
            .route("/{name}", web::get().to(greet))
            .app_data(connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

