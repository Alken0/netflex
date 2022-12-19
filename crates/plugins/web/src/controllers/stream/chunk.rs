use axum::{
    body::{self, Full},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::logic::stream::Chunk;

#[derive(Debug)]
pub struct ChunkResponse(Chunk);

impl ChunkResponse {
    pub fn new(chunk: Chunk) -> Self {
        ChunkResponse(chunk)
    }
}

impl IntoResponse for ChunkResponse {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::PARTIAL_CONTENT)
            .header("Content-Type", self.0.mime)
            .header("Accept-Ranges", "bytes")
            .header(
                "Content-Range",
                format!("bytes {}-{}/{}", self.0.start, self.0.end, self.0.file_size),
            )
            .body(body::boxed(Full::from(self.0.content)))
            .unwrap()
    }
}
