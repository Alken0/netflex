use axum::Router;
mod audios;
mod files;
mod index;
mod settings;
mod statics;
mod stream;
mod videos;

pub fn setup(router: Router) -> Router {
    router
        .nest("/settings", settings::setup())
        .nest("/static", statics::setup())
        .nest("/api/stream", stream::setup())
        .nest("/files", files::setup())
        .nest("/videos", videos::setup())
        .nest("/audios", audios::setup())
        .fallback(index::setup())
}
