use crate::error::Result;
use crate::persistence::db::Connection;
use crate::persistence::repos::file::{File, FindByHash};

#[derive(Clone)]
pub struct FindByHashAction<Conn> {
    conn: Conn,
}

impl<Conn: Connection> FindByHashAction<Conn>
where
    Conn::Transaction: FindByHash,
{
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }

    pub async fn execute(self, hash: u64) -> Result<Option<File>> {
        let transaction = self.conn.transaction().await?;
        return transaction.find_by_hash(hash).await;
    }
}
