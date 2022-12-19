use crate::Transaction;
use async_trait::async_trait;
use domain::{
    error::{database, Result},
    persistence::db::Connection,
};
use sea_orm::{ConnectionTrait, DatabaseConnection, Schema, TransactionTrait};

#[derive(Clone)]
pub struct Database(pub DatabaseConnection);

impl Database {
    pub async fn new(url: &str) -> Self {
        let conn = sea_orm::Database::connect(url).await.unwrap();
        Self(conn)
    }

    pub async fn migrate(&self) -> Result<()> {
        let builder = self.0.get_database_backend();
        let schema = Schema::new(builder);

        let mut statement = schema.create_table_from_entity(crate::file::entity::Entity);
        let statement = statement.if_not_exists();
        let statement = builder.build(statement);

        self.0
            .execute(statement)
            .await
            .map(|_e| ())
            .map_err(|e| database(e.to_string()))
    }
}

#[async_trait]
impl Connection for Database {
    type Transaction = Transaction;

    async fn transaction(&self) -> Result<Self::Transaction> {
        self.0
            .begin()
            .await
            .map(Transaction)
            .map_err(|e| database(e.to_string()))
    }
}
