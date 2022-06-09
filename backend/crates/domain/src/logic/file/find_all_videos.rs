use crate::error::Result;
use crate::persistence::db::Connection;
use crate::persistence::repos::file::File;
use crate::persistence::repos::file::FindAllWhereMimeLikeAny;

const STREAMABLE_VIDEO_MIMES: &[&str] = &[
    "video/webm",
    "video/ogg",
    "video/mp4",
    "video/mpeg",
    "video/x-matroska",
    "video/x-wav",
];

#[derive(Clone)]
pub struct FindAllVideosAction<Conn> {
    conn: Conn,
}

impl<Conn: Connection> FindAllVideosAction<Conn>
where
    Conn::Transaction: FindAllWhereMimeLikeAny,
{
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }

    pub async fn execute(&self) -> Result<Vec<File>> {
        let transaction = self.conn.transaction().await?;
        transaction
            .find_all_where_mime_like_any(STREAMABLE_VIDEO_MIMES)
            .await
    }
}
