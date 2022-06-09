use crate::error::Result;
use crate::persistence::db::Connection;
use crate::persistence::repos::file::File;
use crate::persistence::repos::general::FindAll;

#[derive(Clone)]
pub struct FindAllAction<Conn> {
    conn: Conn,
}

impl<Conn: Connection> FindAllAction<Conn>
where
    Conn::Transaction: FindAll<File>,
{
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }

    pub async fn execute(&self) -> Result<Vec<File>> {
        let transaction = self.conn.transaction().await?;
        return transaction.find_all().await;
    }
}
