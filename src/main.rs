use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    let server = zero2prod::startup::run(listener).await?;
    let _ = tokio::spawn(server).await;
    Ok(())
}
