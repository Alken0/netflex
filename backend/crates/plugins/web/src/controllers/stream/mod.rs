use self::{chunk::ChunkResponse, range::HeaderRange};
use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Router};
use domain::{logic::stream::StreamAction, types::Id};
use persistence::Database;

mod chunk;
mod range;

pub fn setup() -> Router {
    Router::new().route("/:id", get(stream))
}

async fn stream(
    Extension(action): Extension<StreamAction<Database>>,
    Path(id): Path<Id>,
    range: HeaderRange,
) -> impl IntoResponse {
    let chunk = action.execute(id, range.into()).await.unwrap();
    let chunk = ChunkResponse::new(chunk);
    return chunk;
}
