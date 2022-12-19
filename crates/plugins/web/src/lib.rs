use axum::Router;
use config::ServerConfig;
use domain::{config::Config, error::Result};
use persistence::Database;

pub mod config;
mod controllers;
mod extensions;
mod templates;

pub async fn app(server_config: ServerConfig, config: Config) -> Result<Router> {
    let db = Database::new(&server_config.db_url).await;
    db.migrate().await.unwrap();

    let mut router = Router::new();

    router = controllers::setup(router, &server_config);
    router = extensions::setup(router, &db, &config, &server_config); // has to come after controllers

    Ok(router)
}
