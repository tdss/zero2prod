use serde::{Serialize,Deserialize};
use actix_web::Responder;
use uuid::Uuid;
use actix_web::web;

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    username: String,
    email: String
}

#[derive(Serialize)]
pub struct PlayerResponseData {
    username: String,
    id: uuid::Uuid
}

pub async fn play(req: web::Query<PlayerData>) -> actix_web::Result<impl Responder> {
    let obj = PlayerResponseData {
        username: req.username.to_string(),
        id: uuid::Uuid::new_v4()
    };
    Ok(web::Json(obj))
}



#[derive(Deserialize)]
pub struct GameRequestData {
    game_name: String,
}

#[derive(Serialize)]
pub struct GameResponseData {
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

pub async fn read_game_data(req: web::Query<GameRequestData>) -> actix_web::Result<impl Responder> {
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


