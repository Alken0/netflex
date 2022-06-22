use super::chunk::Chunk;
use super::Range;
use crate::error::Result;
use crate::persistence::db::Connection;
use crate::persistence::repos::file::File;
use crate::{error::not_found, persistence::repos::file::FindByPath};

#[derive(Clone)]
pub struct StreamAction<Conn> {
    conn: Conn,
}

impl<Conn: Connection> StreamAction<Conn>
where
    Conn::Transaction: FindByPath,
{
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }

    pub async fn execute(&self, path: &str, range: Option<Range>) -> Result<Chunk> {
        let file = self.find_file_in_db(path).await?;
        let range = range.unwrap_or_default();
        let chunk = Chunk::new(file, &range).await?;
        return Ok(chunk);
    }

    async fn find_file_in_db(&self, path: &str) -> Result<File> {
        self.conn
            .transaction()
            .await?
            .find_by_path(path)
            .await?
            .ok_or_else(|| not_found("File not found"))
    }
}
