use axum::http::StatusCode;
use axum::routing::get_service;
use axum::Router;
use tower_http::services::ServeDir;

pub fn setup() -> Router {
    Router::new().nest(
        "",
        get_service(ServeDir::new("crates/plugins/web/static")).handle_error(|error| async move {
            (
                StatusCode::NOT_FOUND,
                format!("Ressource not found: {}", error),
            )
        }),
    )
}
