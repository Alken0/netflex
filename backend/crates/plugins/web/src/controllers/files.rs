use axum::response::Html;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use domain::{logic::file::find_all::FindAllAction, persistence::repos::file::File};
use persistence::Database;
use serde::Serialize;
use tera::Context;

pub fn setup() -> Router {
    Router::new().route("/", get(list_files))
}

#[derive(Debug, Serialize)]
struct TemplateContext {
    files: Vec<File>,
}

async fn list_files(Extension(action): Extension<FindAllAction<Database>>) -> Html<String> {
    let files = action.execute().await.unwrap();
    let context = Context::from_serialize(TemplateContext { files }).unwrap();
    return crate::templates::parse("views/files.html", &context);
}
