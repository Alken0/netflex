use axum::Router;

use crate::config::ServerConfig;
mod audios;
mod files;
mod index;
mod settings;
mod statics;
mod stream;
mod videos;

pub fn setup(router: Router, config: &ServerConfig) -> Router {
    router
        .nest("/settings", settings::setup())
        .nest("/static", statics::setup(&config.statics_path))
        .nest("/api/stream", stream::setup())
        .nest("/files", files::setup())
        .nest("/videos", videos::setup())
        .nest("/audios", audios::setup())
        .fallback(index::setup())
}
