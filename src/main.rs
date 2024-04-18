use std::net::TcpListener;


use sqlx::PgPool;
use zero2prod::telemetry::{init_subscriber, get_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    let subscriber = get_subscriber("zero2prod".into(), "into".into());
    init_subscriber(subscriber);
    let configuration = zero2prod::configuration::get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await.expect("Failed to connect to Postgres");
    let server = zero2prod::startup::run(listener, connection_pool).await?;
    let _ = tokio::spawn(server).await;
    Ok(())
}
