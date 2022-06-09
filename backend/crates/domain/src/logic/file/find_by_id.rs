use crate::persistence::repos::file::File;
use crate::{error::Result, persistence::repos::general::FindById};
use crate::{persistence::db::Connection, types::Id};

#[derive(Clone)]
pub struct FindByIdAction<Conn> {
    conn: Conn,
}

impl<Conn: Connection> FindByIdAction<Conn>
where
    Conn::Transaction: FindById<File>,
{
    pub fn new(conn: Conn) -> Self {
        Self { conn }
    }

    pub async fn execute(&self, id: Id) -> Result<Option<File>> {
        let transaction = self.conn.transaction().await?;
        return transaction.find_by_id(id).await;
    }
}
