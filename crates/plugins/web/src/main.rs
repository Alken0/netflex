use axum::Server;
use domain::error::Result;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;
use web::config::ServerConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config();

    let app = web::app(config.server, config.domain).await?;

    Server::bind(&address(&config.address))
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone, Default, Deserialize, Serialize)]
struct Config {
    address: String,
    server: ServerConfig,
    domain: domain::config::Config,
}

fn config() -> Config {
    confy::load_path("./config-local.toml").unwrap()
}

fn address(address: &str) -> SocketAddr {
    SocketAddr::from_str(address).unwrap()
}
