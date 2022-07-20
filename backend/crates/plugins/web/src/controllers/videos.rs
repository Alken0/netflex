use axum::routing::get;
use axum::Extension;
use axum::Router;
use axum::{extract::Path, response::Html};
use domain::logic::file::find_all_videos::FindAllVideosAction;
use domain::{logic::file::find_by_hash::FindByHashAction, persistence::repos::file::File};
use persistence::Database;
use serde::Serialize;
use tera::Context;

use crate::templates::Templates;

pub fn setup() -> Router {
    Router::new()
        .route("/:hash", get(player))
        .route("/", get(list))
}

#[derive(Debug, Serialize)]
struct ListTemplateContext {
    videos: Vec<File>,
}

async fn list(
    Extension(action): Extension<FindAllVideosAction<Database>>,
    Extension(templates): Extension<Templates>,
) -> Html<String> {
    let videos = action.execute().await.unwrap();
    let context = Context::from_serialize(ListTemplateContext { videos }).unwrap();
    return templates.parse("views/videos/list.html", &context);
}

#[derive(Debug, Serialize)]
struct PlayerTemplateContext {
    video: File,
}

async fn player(
    Extension(action): Extension<FindByHashAction<Database>>,
    Path(hash): Path<u64>,
    Extension(templates): Extension<Templates>,
) -> Html<String> {
    let video = action.execute(hash).await.unwrap().unwrap();
    let context = Context::from_serialize(PlayerTemplateContext { video }).unwrap();
    return templates.parse("views/videos/player.html", &context);
}
