use axum::{response::Redirect, routing::get, Router};

pub fn setup() -> Router {
    Router::new().nest("", get(index))
}

async fn index() -> Redirect {
    return Redirect::to("/videos");
}
