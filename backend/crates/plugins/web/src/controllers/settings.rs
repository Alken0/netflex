use axum::response::{Html, Redirect};
use axum::routing::{get, post};
use axum::Extension;
use axum::Router;
use domain::logic::update::{UpdateAction, UpdateConfig};
use persistence::Database;
use tera::Context;

pub fn setup() -> Router {
    Router::new()
        .route("/refresh", post(refresh))
        .route("/shutdown", post(shutdown))
        .route("/", get(settings))
}

async fn settings() -> Html<String> {
    let context = Context::new();
    return crate::templates::parse("views/settings.html", &context);
}

async fn refresh(Extension(action): Extension<UpdateAction<Database>>) -> Redirect {
    let config = UpdateConfig {
        exclude_dirs: Vec::new(),
        paths: vec!["./data".to_string()],
    };
    tokio::task::spawn(async move {
        action.execute(config).await.unwrap();
    });
    Redirect::to("/")
}

async fn shutdown() {
    std::process::exit(0)
}
