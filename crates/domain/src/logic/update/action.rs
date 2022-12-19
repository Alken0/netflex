use super::util::reader;
use crate::persistence::repos::file::File;
use crate::persistence::repos::file::FileInsert;
use crate::persistence::repos::general::DeleteAll;
use crate::persistence::repos::general::InsertAll;
use crate::{config::Config, persistence::db::Connection};
use crate::{error::Result, persistence::db::Transaction};

#[derive(Clone)]
pub struct UpdateAction<Conn> {
    conn: Conn,
    config: Config,
}

impl<Conn: Connection> UpdateAction<Conn>
where
    Conn::Transaction: DeleteAll<File> + InsertAll<FileInsert>,
{
    pub fn new(conn: Conn, config: Config) -> Self {
        Self { conn, config }
    }

    pub async fn execute(self) -> Result<()> {
        let transaction = self.conn.transaction().await?;
        self.delete_old_data(&transaction).await?;
        self.run_update(&transaction).await?;
        transaction.commit().await?;
        Ok(())
    }

    async fn run_update<T: InsertAll<FileInsert>>(self, transaction: &T) -> Result<()> {
        let mut dirs_to_check = self.config.paths_to_check;
        while !dirs_to_check.is_empty() {
            let content = reader::extract(&dirs_to_check).await;

            let files = content.files.into_iter().map(Into::into).collect();
            transaction.insert_all(files).await?;

            dirs_to_check = content.dirs.into_iter().map(|e| e.path()).collect();
        }
        Ok(())
    }

    async fn delete_old_data<T: DeleteAll<File>>(&self, transaction: &T) -> Result<()> {
        transaction.delete_all().await?;
        Ok(())
    }
}
