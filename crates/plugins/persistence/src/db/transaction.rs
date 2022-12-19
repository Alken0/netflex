use async_trait::async_trait;
use domain::error::{database, Result};
use sea_orm::DatabaseTransaction;

pub struct Transaction(pub(crate) DatabaseTransaction);

#[async_trait]
impl domain::persistence::db::Transaction for Transaction {
    async fn commit(self) -> Result<()> {
        self.0.commit().await.map_err(|e| database(e.to_string()))
    }
}
