use axum::response::{Html, Redirect};
use axum::routing::{get, post};
use axum::Extension;
use axum::Router;
use domain::logic::update::UpdateAction;
use persistence::Database;
use tera::Context;

use crate::templates::Templates;

pub fn setup() -> Router {
    Router::new()
        .route("/refresh", post(refresh))
        .route("/shutdown", post(shutdown))
        .route("/", get(settings))
}

async fn settings(Extension(templates): Extension<Templates>) -> Html<String> {
    let context = Context::new();
    return templates.parse("views/settings.html", &context);
}

async fn refresh(Extension(action): Extension<UpdateAction<Database>>) -> Redirect {
    tokio::task::spawn(async move {
        action.execute().await.unwrap();
    });
    Redirect::to("/")
}

async fn shutdown() {
    std::process::exit(0)
}
