use actix_web::HttpResponse;
use serde::{Serialize,Deserialize};
use actix_web::Responder;
use sqlx::PgPool;
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
    title: String,
    description: String,
    reward: String,
    monetary_reward: i32,
    monetary_reward_increase: i32,
    created_at: chrono::DateTime<chrono::Utc>,
    launched_at: chrono::DateTime<chrono::Utc>,
    id: uuid::Uuid,
    time_reset_period: i32,
    max_number_of_players: i32,
}

#[derive(Serialize)] 
pub struct GameWinnerData {
    id: uuid::Uuid,
    current_winner: PlayerResponseData,
    previous_winners: Vec<PlayerResponseData>
}

//pub async fn get_game(req: web::Query<GameRequestData>) -> actix_web::Result<impl Responder> {
//    match 
//}
#[tracing::instrument(
    name="Getting game from database"
    skip(connection, req)
)]
pub async fn read_game_data(req: web::Query<GameRequestData>, connection: web::Data<PgPool>) -> actix_web::Result<impl Responder> {
    let mut transaction = connection.begin().await.unwrap();
    get_game(&connection).await;
    let x: &PgPool = &connection;
    let data = sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        uuid::Uuid::new_v4(),
        "x".to_string(),
        "y".to_string(),
        chrono::Utc::now()
    ).execute(connection.get_ref()).await;//.expect("No game in database");


    match data {
            Ok(_) => Ok(web::Json({})),
            Err(_) => Ok(web::Json({})),
        }
 
}

pub async fn get_game(pool: &PgPool) -> actix_web::Result<impl Responder> {
    let data = sqlx::query!(
    r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        uuid::Uuid::new_v4(),
        "x".to_string(),
        "y".to_string(),
        chrono::Utc::now()
    ).execute(pool).await;//.expect("No game in database");


    match data {
            Ok(_) => Ok(web::Json({})),
            Err(_) => Ok(web::Json({})),
        }
   
}

    /*.map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        ErrorNotFound(e)
    });
    */


    /*let obj = GameResponseData {
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
    Ok(web::Json(obj))*/
//}


