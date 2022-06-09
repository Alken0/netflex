use axum::Router;
use domain::error::Result;
use persistence::Database;

mod controllers;
mod extensions;
mod templates;

pub async fn app(db: &Database) -> Result<Router> {
    db.migrate().await.unwrap();

    let mut router = Router::new();

    router = controllers::setup(router);
    router = extensions::setup(router, db); // has to come after controllers

    Ok(router)
}
