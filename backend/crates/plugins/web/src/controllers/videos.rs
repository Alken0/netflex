use axum::routing::get;
use axum::Extension;
use axum::Router;
use axum::{extract::Path, response::Html};
use domain::persistence::repos::file::File;
use domain::{
    logic::file::{find_all_videos::FindAllVideosAction, find_by_id::FindByIdAction},
    types::Id,
};
use persistence::Database;
use serde::Serialize;
use tera::Context;

pub fn setup() -> Router {
    Router::new()
        .route("/:id", get(player))
        .route("/", get(list))
}

#[derive(Debug, Serialize)]
struct ListTemplateContext {
    videos: Vec<File>,
}

async fn list(Extension(action): Extension<FindAllVideosAction<Database>>) -> Html<String> {
    let videos = action.execute().await.unwrap();
    let context = Context::from_serialize(ListTemplateContext { videos }).unwrap();
    return crate::templates::parse("views/videos/list.html", &context);
}

#[derive(Debug, Serialize)]
struct PlayerTemplateContext {
    video: File,
}

async fn player(
    Extension(action): Extension<FindByIdAction<Database>>,
    Path(id): Path<Id>,
) -> Html<String> {
    let video = action.execute(id).await.unwrap().unwrap();
    let context = Context::from_serialize(PlayerTemplateContext { video }).unwrap();
    return crate::templates::parse("views/videos/player.html", &context);
}
