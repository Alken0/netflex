use super::Range;
use crate::error::os_error;
use crate::error::Result;
use crate::persistence::repos::file::File;
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, AsyncSeekExt, Result as TokioResult, SeekFrom};

type Bytes = Vec<u8>;

#[derive(Debug)]
pub struct Chunk {
    pub start: u64,
    pub end: u64,
    pub file_size: u64,
    pub mime: String,
    pub content: Bytes,
}

impl Chunk {
    pub(crate) async fn new(file: File, range: &Range) -> Result<Self> {
        let range = range.set_max_end(file.size);

        let content = chunk(file.path, &range)
            .await
            .map_err(|e| os_error(&format!("problem reading file: {e}")))?;

        Ok(Self {
            start: range.start(),
            end: range.end().saturating_sub(1),
            file_size: file.size,
            mime: file.mime,
            content,
        })
    }
}

async fn chunk(path: String, range: &Range) -> TokioResult<Bytes> {
    let mut buffer = Bytes::new();
    let mut file = TokioFile::open(path).await?;

    file.seek(SeekFrom::Start(range.start())).await?;
    file.take(range.limit()).read_to_end(&mut buffer).await?;

    return Ok(buffer);
}
