use axum::http::StatusCode;
use axum::routing::get_service;
use axum::Router;
use tower_http::services::ServeDir;

pub fn setup(statics_path: &str) -> Router {
    Router::new().nest(
        "",
        get_service(ServeDir::new(statics_path)).handle_error(|error| async move {
            (
                StatusCode::NOT_FOUND,
                format!("Ressource not found: {}", error),
            )
        }),
    )
}
