use self::{chunk::ChunkResponse, range::HeaderRange};
use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Router};
use domain::logic::stream::StreamAction;
use persistence::Database;

mod chunk;
mod range;

pub fn setup() -> Router {
    Router::new().route("/*path", get(stream))
}

async fn stream(
    Extension(action): Extension<StreamAction<Database>>,
    Path(path): Path<String>,
    range: HeaderRange,
) -> impl IntoResponse {
    let path = &path[1..]; //remove leading slash
    let chunk = action.execute(&path, range.into()).await.unwrap();
    let chunk = ChunkResponse::new(chunk);
    return chunk;
}
