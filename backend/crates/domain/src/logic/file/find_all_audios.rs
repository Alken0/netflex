use crate::error::Result;
use crate::persistence::db::Connection;
use crate::persistence::repos::file::File;
use crate::persistence::repos::file::FindAllWhereMimeLikeAny;

const STREAMABLE_AUDIO_MIMES: &[&str] = &[
    "audio/webm",
    "audio/ogg",
    "audio/mp3",
    "audio/x-matroska",
    "audio/mpeg",
];

#[derive(Clone)]
pub struct FindAllAudiosAction<Conn> {
    conn: Conn,
}

impl<Conn: Connection> FindAllAudiosAction<Conn>
where
    Conn::Transaction: FindAllWhereMimeLikeAny,
{
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }

    pub async fn execute(self) -> Result<Vec<File>> {
        let transaction = self.conn.transaction().await?;
        transaction
            .find_all_where_mime_like_any(STREAMABLE_AUDIO_MIMES)
            .await
    }
}
