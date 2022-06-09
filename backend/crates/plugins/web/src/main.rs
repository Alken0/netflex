use axum::Server;
use domain::error::Result;
use persistence::Database;
use std::net::SocketAddr;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let db_url = "sqlite://sqlite.db"; //"postgresql://postgres:password@localhost"; //"postgresql:db.postgresql"; //"sqlite://db.sqlite"
    let db = Database::new(db_url).await;

    let app = web::app(&db).await?;

    let address = SocketAddr::from_str("127.0.0.1:8080").unwrap();
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
